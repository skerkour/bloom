.PHONY: all
all: build

.PHONY: build
build:
	mkdir -p dist
	npm run build
	cp demo.html dist/index.html


.PHONY: dev
dev:
	gowatch -log-prefix=false -build="make build" -pattern ".+\\.js" -exclude-dir="dist"


.PHONY: clean
clean:
	rm -rf dist


.PHONY: serve
serve:
	python3 -m http.server 8001 --directory dist


.PHONY: install
install:
	npm install


.PHONY: re
re: clean build
