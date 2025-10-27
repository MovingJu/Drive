#!/bin/sh
set -e 

DOCKER_PLATFORM="linux/amd64,linux/arm64"
DOCKER_IMAGE_NAME="movingju/public:drive"

for arg in "$@"; do
    case "$arg" in
        docker-platform=*) DOCKER_PLATFORM="${arg#*=}" ;;
        *) ;;
    esac
done

docker_build(){
    docker buildx build \
    --platform $DOCKER_PLATFORM \
    -t $DOCKER_IMAGE_NAME \
    .
}

docker_push(){
    docker push $DOCKER_IMAGE_NAME
}

for arg in "$@"; do
    case "$arg" in
        docker-build) docker_build ;;
        docker-push) docker_push ;;
        docker-platform=*) ;;
        *)
            echo "Unknown command: $arg"
            exit 1
            ;;
    esac
done
