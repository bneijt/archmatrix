Matrix of Archlinux based docker images
========================

This is a combination of features in a docker image. Each feature is part of the tag name using snake case:

- **Pyenv39**: Python 3.9 installed using pyenv
- **Stipped**: deleted most extra files on the filesystem, nolonger usable to continue development as the shell has been removed.

The docker files in `tags/` are generated by a Rust program. See [`generate.sh`](generate.sh) for more information.