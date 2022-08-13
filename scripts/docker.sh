#! /bin/bash
set -e

#ls
echo v1 $1
echo v12 $2
echo v11 $3
docker buildx build \
#  --target scratch \
#  --platform linux/amd64,linux/arm64 . \
#  -t europe-west1-docker.pkg.dev/maya-mussia/nx-go-playground/rust-users-api \
#  --push \
