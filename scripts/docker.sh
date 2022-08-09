#! /bin/bash
set -e

ls
docker buildx build \
  --target scratch \
  --push \
  --platform linux/amd64,linux/arm64 . \
  -t europe-west1-docker.pkg.dev/maya-mussia/nx-go-playground/rust-users-api:"$TAG" \
