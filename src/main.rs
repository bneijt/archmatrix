use indoc::indoc;
use std::ops::RangeBounds;
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
    let mut pre_builds: Vec<String> = Vec::new();
    let mut joiners: Vec<String> = Vec::new();


    if args.include.contains(&String::from("Pyenv39")) {
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
    }
    let mut dockerfile_body: String = String::new();
    for pre_build in pre_builds {
        dockerfile_body.push_str(&pre_build);
    }
    dockerfile_body.push_str("FROM archlinux:base\n");

    for joiner in joiners {
        dockerfile_body.push_str(&joiner);
    }
    let tag= String::from("pyenv");
    let docker_filename = format!("tags/Dockerfile.{tag}");
    fs::write(docker_filename, dockerfile_body).expect("Failed to write docker file");
}
