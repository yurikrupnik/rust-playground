#! /bin/bash
set -e

ls
docker buildx build \
  --target scratch \
  --build-arg DIST_PATH=target/release/users \
  --push \
  --platform linux/amd64,linux/arm64 . \
  -t yurikrupnik/rust-users-api:"$TAG" \
  -t europe-west1-docker.pkg.dev/maya-mussia/nx-go-playground/rust-users-api:"$TAG" \
