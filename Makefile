build: src/app.rs src/lib.rs
	wasm-pack build --target web

bundle: build
	rollup ./main.js --format iife --file ./pkg/bundle.js

serve: bundle
	python3 -m http.server 8000
