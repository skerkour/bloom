.PHONY: build
build:
	npm run build
	rm -rf dist/demo.html
	cp demo.html dist/index.html
	cp dist/chatbox.umd.min.js dist/chatbox.js


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


.PHONY: serve
serve:
	python3 -m http.server 8002 --directory dist
