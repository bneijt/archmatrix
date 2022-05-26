tags="RnPyenv39 CiAwsTf12"
for tag in $tags; do
    mkdir $tag
    cargo run -- --tag $tag > $tag/Dockerfile
    docker build --file $tag/Dockerfile context
done