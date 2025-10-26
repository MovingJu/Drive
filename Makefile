docker_target = linux/amd64,linux/arm64

docker-build:
	docker buildx build \
		--platform $(docker_target) \
		-t movingju/public:rust_server \
		.