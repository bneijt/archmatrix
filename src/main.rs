use indoc::indoc;
use std::fs;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Tag to produce
    #[clap(short, long, multiple_values = true)]
    include: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let mut tags = args.include.clone();
    tags.sort();
    let mut pre_builds: Vec<String> = Vec::new();
    let mut joiners: Vec<String> = Vec::new();
    let mut post_builds: Vec<String> = Vec::new();
    let mut entrypoint = String::from("/bin/bash");

    if tags.contains(&String::from("Pyenv39")) {
        // TODO determine latest pyenv version online
        let pyenv_version = "3.9.1";
        let pyenv_pre_build = indoc! {r#"
        FROM archlinux:base-devel AS python-base
        
        RUN pacman --noconfirm -Sy; \
            pacman --noconfirm -S archlinux-keyring; \
            pacman --noconfirm -S pyenv
        
        ENV PYENV_ROOT=/pyenv
        
        RUN pyenv install "PYTHON_VERSION" \
            && pyenv global "PYTHON_VERSION"
        
        # Drop cache and linking files
        RUN find /pyenv -type d -a \( -name __pycache__ -o -name test -o -name tests -o -name idle_test \) -exec rm -rf '{}' + \
            && find /pyenv -type f -name '*.a' -exec rm -rf '{}' +
        "#};

        pre_builds.push(pyenv_pre_build.replace("PYTHON_VERSION", pyenv_version));
        let pyenv_joiner = indoc! {r#"
        COPY --from=python-base /pyenv /pyenv
        ENV PATH="/pyenv/versions/PYTHON_VERSION/bin:${PATH}"
        "#};
        joiners.push(pyenv_joiner.replace("PYTHON_VERSION", pyenv_version));
        entrypoint = format!("/pyenv/versions/{pyenv_version}/bin/python");
    }

    if args.include.contains(&String::from("Stripped")) {
        // Check sizes by installing expac and running expac "%n %m" -l'\n' -Q $(pacman -Qq) | sort -rhk 2
        let to_drop = vec![
            "acl",
            "archlinux-keyring",
            "argon2",
            // "attr",
            // "audit",
            // "base",
            "bash",
            // "brotli",
            // "bzip2",
            // "ca-certificates",
            // "ca-certificates-mozilla",
            // "ca-certificates-utils",
            "coreutils",
            "cryptsetup",
            "curl",
            // "dbus",
            // "device-mapper",
            "e2fsprogs",
            // "expat",
            "file",
            // "filesystem",
            "findutils",
            "gawk",
            // "gcc-libs",
            // "gdbm",
            // "gettext",
            // "glib2",
            // "glibc",
            // "gmp",
            "gnupg",
            // "gnutls",
            // "gpgme",
            "grep",
            // "gzip",
            // "hwdata",
            // "iana-etc",
            // "icu",
            "iproute2",
            "iptables",
            "iputils",
            // "json-c",
            // "kbd",
            "keyutils",
            // "kmod",
            "krb5",
            "less",
            // "libarchive",
            // "libassuan",
            // "libbpf",
            // "libcap",
            // "libcap-ng",
            // "libelf",
            // "libffi",
            // "libgcrypt",
            // "libgpg-error",
            // "libidn2",
            // "libksba",
            // "libldap",
            // "libmnl",
            // "libnetfilter_conntrack",
            // "libnfnetlink",
            // "libnftnl",
            // "libnghttp2",
            // "libnl",
            // "libp11-kit",
            // "libpcap",
            // "libpsl",
            // "libsasl",
            // "libseccomp",
            // "libsecret",
            // "libssh2",
            // "libsysprof-capture",
            // "libtasn1",
            // "libtirpc",
            // "libunistring",
            // "libxcrypt",
            "libxml2",
            "licenses",
            "linux-api-headers",
            // "lz4",
            // "mpfr",
            "ncurses",
            "nettle",
            // "npth",
            // "openssl",
            "p11-kit",
            // "pacman",
            "pacman-mirrorlist",
            "pam",
            "pambase",
            "pciutils",
            // "pcre",
            // "pcre2",
            "pinentry",
            "popt",
            "procps-ng",
            // "psmisc",
            // "readline",
            "sed",
            "shadow",
            "sqlite",
            "systemd",
            "systemd-libs",
            "systemd-sysvcompat",
            "tar",
            "tpm2-tss",
            // "tzdata",
            "util-linux",
            "util-linux-libs",
            "xz",
            // "zlib",
            // "zstd",
        ];
        let drop_args = to_drop.join(" ");
        joiners.push(format!("RUN pacman --noconfirm -Rndd {drop_args}"));
        post_builds.push(
            indoc! {r#"
        FROM scratch
        COPY --from=joiner / /
        "#}
            .into(),
        );
    }

    // Build docker files
    let mut dockerfile_body: String = String::new();
    for pre_build in pre_builds {
        dockerfile_body.push_str(&pre_build);
        dockerfile_body.push('\n');
    }
    dockerfile_body.push_str("FROM archlinux:base AS joiner");
    dockerfile_body.push('\n');

    for joiner in &joiners {
        dockerfile_body.push_str(joiner);
        dockerfile_body.push('\n');
    }
    dockerfile_body.push_str(&format!("ENTRYPOINT [\"{entrypoint}\"]"));
    dockerfile_body.push('\n');

    for post_build in &post_builds {
        dockerfile_body.push_str(&post_build);
        dockerfile_body.push('\n');
    }
    if !post_builds.is_empty() {
        dockerfile_body.push_str(&format!("ENTRYPOINT [\"{entrypoint}\"]"));
        dockerfile_body.push('\n');
    }
    let tag = args.include.join("");
    let docker_filename = format!("tags/Dockerfile.{tag}");
    fs::write(docker_filename, dockerfile_body).expect("Failed to write docker file");
}
