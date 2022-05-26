Matrix of Archlinux based docker images
========================

This is a combination of features in a docker image. Each feature is part of the tag name using snake case:

- Ci: meant for CI pipelines.
- Tf: Terraform.
- Aws: AWS cli installed.
- Run: meant for runtime.
- Stipper: Ready to strip image.
- Stipped: Stripped of any extra files on the filesystem, nolonger usable to continue development.
- Pyenv39: Python 3.9 installed using pyenv

We build a matrix combination of these images using a Rust script.
