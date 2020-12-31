NAME = bloom
VERSION = $(shell cat package.json | grep version | cut -d '"' -f4)
DATE = $(shell date +"%Y-%m-%d")
DIST_PUBLIC_DIR = dist

.PHONY: build
build:
	npm run build


.PHONY: install
install:
	npm install


.PHONY: dev
dev:
	npm run serve


.PHONY: clean
clean:
	rm -rf dist


.PHONY: lint
lint:
	npm run lint


.PHONY: re
re: clean build


.PHONY: libs
libs:
	mkdir -p $(DIST_PUBLIC_DIR)/libs
	cp -r ../bloom.js/dist/bloom.js $(DIST_PUBLIC_DIR)/libs
	cp -r ../chatbox.js/dist/chatbox.js $(DIST_PUBLIC_DIR)/libs
