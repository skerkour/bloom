NAME = bloom
COMMIT = $(shell git rev-parse HEAD)
DATE := $(shell date +"%Y-%m-%d")
DIST_DIR = dist
DOCKER_IMAGE = ghcr.io/skerkour/bloom-legacy
TARGET_DIR = target
RELEASE_BINARY = target/release/$(NAME)
PUBLIC_DIR = public


.PHONY: all
all: docker


.PHONY: release
release:
	git checkout release
	git merge main
	git push
	git checkout main


.PHONY: webapp
webapp:
	make -C webapp


.PHONY: bloom.js
bloom.js:
	make -C bloom.js


.PHONY: chatbox.js
chatbox.js:
	make -C chatbox.js


.PHONY: docker
docker:
	docker build -t $(DOCKER_IMAGE):latest .


.PHONY: docker_release
docker_release:
	docker push $(DOCKER_IMAGE):latest
