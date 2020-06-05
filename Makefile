build:
	wasm-pack build --target web
	rollup ./main.js --format iife --file ./pkg/bundle.js
	cp .htaccess ./pkg

serve: build
	python3 -m http.server 8000

clean:
	rm -R ./pkg/*
