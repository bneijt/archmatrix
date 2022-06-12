use indoc::indoc;
use std::fs;
mod pyenv;
use clap::Parser;
use simple_error::SimpleError;
use std::fmt;
use strum::EnumString;

#[derive(Debug, EnumString, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum MatrixFeature {
    Pyenv39,
    Tf12,
    A,
    Stripped,
    Aws,
}

impl fmt::Display for MatrixFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

//https://gitlab.com/gitlab-org/cloud-deploy/-/blob/master/aws/base/Dockerfile
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Tag to produce
    #[clap(short, long, multiple_values = true)]
    include: Vec<MatrixFeature>,
}

#[tokio::main]
async fn main() -> Result<(), Box<SimpleError>> {
    let args = Args::parse();
    let mut tags = args.include.clone();
    tags.sort();
    let mut pre_builds: Vec<String> = Vec::new();
    let mut joiners: Vec<String> = Vec::new();
    let mut post_builds: Vec<String> = Vec::new();
    let mut entrypoint = String::from("/bin/bash");
    if tags.contains(&MatrixFeature::Aws) {
        joiners.push( indoc! {r#"
        RUN pacman --noconfirm -Sy archlinux-keyring \
            && pacman --noconfirm -S aws-cli \
            && pacman --noconfirm -Scc
        "#}.to_string());
    }
    if tags.contains(&MatrixFeature::Pyenv39) {
        // TODO determine latest pyenv version online
        // by parsing https://api.github.com/repos/pyenv/pyenv/git/trees/master?recursive=true
        // and extracting the correct versions from https://github.com/pyenv/pyenv/tree/master/plugins/python-build/share/python-build
        // use https://api.github.com/repos/pyenv/pyenv/contents/plugins/python-build/share/python-build
        let pyenv_version = pyenv::load_latest_with_prefix(&String::from("3.9")).await?;
        let pyenv_pre_build = indoc! {r#"
        FROM archlinux:base-devel AS python-base
        
        RUN pacman --noconfirm -Sy \
            && pacman --noconfirm -S archlinux-keyring \
            && pacman --noconfirm -S pyenv \
            && pacman --noconfirm -Scc
        
        ENV PYENV_ROOT=/pyenv
        
        RUN pyenv install "PYTHON_VERSION" \
            && pyenv global "PYTHON_VERSION"
        
        # Drop cache and linking files
        RUN find /pyenv -type d -a \( -name __pycache__ -o -name test -o -name tests -o -name idle_test \) -exec rm -rf '{}' + \
            && find /pyenv -type f -name '*.a' -exec rm -rf '{}' +
        "#};

        pre_builds.push(pyenv_pre_build.replace("PYTHON_VERSION", &pyenv_version));
        let pyenv_joiner = indoc! {r#"
        COPY --from=python-base /pyenv /pyenv
        ENV PATH="/pyenv/versions/PYTHON_VERSION/bin:${PATH}"
        "#};
        joiners.push(pyenv_joiner.replace("PYTHON_VERSION", &pyenv_version));
        entrypoint = format!("/pyenv/versions/{pyenv_version}/bin/python");
    }
    if tags.contains(&MatrixFeature::Tf12) {
        let tf_version = "1.2.1";
        let terraform_pre_build = indoc! {r#"
        FROM archlinux:base-devel AS tf1-base
        
        RUN pacman --noconfirm -Sy archlinux-keyring \
            && pacman --noconfirm -S unzip \
            && pacman --noconfirm -Scc

        RUN curl -sLo terraform.zip "https://releases.hashicorp.com/terraform/TERRAFORM_VERSION/terraform_TERRAFORM_VERSION_linux_amd64.zip" \
            && unzip terraform.zip \
            && rm terraform.zip \
            && mv ./terraform /usr/local/bin/terraform-TERRAFORM_VERSION \
            && ln -s /usr/local/bin/terraform-TERRAFORM_VERSION /usr/local/bin/terraform \
            && terraform --version
        "#};
        let tf_joiner = indoc! {r#"
        COPY --from=tf1-base /usr/local/bin/terraform* /usr/local/bin/
        "#};
        pre_builds.push(terraform_pre_build.replace("TERRAFORM_VERSION", &tf_version));
        joiners.push(tf_joiner.to_string());
    }

    if tags.contains(&MatrixFeature::A) {
        joiners.push(String::from("RUN mkdir /app && useradd --home-dir /app --no-create-home --shell /usr/bin/nologin app"));
        joiners.push(String::from("WORKDIR app"));
        joiners.push(String::from("USER app"));
    }

    if args.include.contains(&MatrixFeature::Stripped) {
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
            // "ncurses", // Breaks /bin/sh
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
        joiners.push(format!("RUN rm -rf /usr/share/info"));
        joiners.push(format!("RUN rm -rf /usr/include"));
        joiners.push(format!("RUN rm -rf /usr/lib/*.a"));

        // Will break subsequent RUN commands
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
    let tag = args
        .include
        .iter()
        .map(|x| x.to_string())
        .collect::<String>();
    let docker_filename = format!("tags/Dockerfile.{tag}");
    fs::write(docker_filename, dockerfile_body).expect("Failed to write docker file");
    Ok(())
}
