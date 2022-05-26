
use std::ops::RangeBounds;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Tag to produce
    #[clap(short, long)]
    tag: String,
}

fn main() {
    let args = Args::parse();
    let mut pre_builds: Vec<&str> = Vec::new();
    let mut joiners: Vec<&str> = Vec::new();
    if args.tag.contains("Pyenv39") {
        pre_builds.push(r#"
        FROM archlinux:base-devel AS python-base
        
        RUN pacman --noconfirm -Sy; \
            pacman --noconfirm -S archlinux-keyring; \
            pacman --noconfirm -S pyenv
        
        ENV PYENV_ROOT=/pyenv
        
        RUN pyenv install "3.9.12" \
            && pyenv global "3.9.12"
        
        # Drop cache and linking files
        RUN find /pyenv -type d -a \( -name __pycache__ -o -name test -o -name tests -o -name idle_test \) -exec rm -rf '{{}}' + \
            && find /pyenv -type f -name '*.a' -exec rm -rf '{{}}' +
        "#);
        joiners.push(r#"
        COPY --from=python-base /pyenv /pyenv
        ENV PATH="/pyenv/versions/3.9.12/bin:${{PATH}}"
        "#);
    }

    for pre_build in pre_builds {
        println!("{}", pre_build);
    }
    println!("FROM archlinux:base");
    for joiner in joiners {
        println!("{}", joiner);
    }


}
