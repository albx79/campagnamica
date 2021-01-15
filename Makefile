build:
	wasm-pack build --target web
	rollup ./main.js --format iife --file ./pkg/bundle.js
	cp .htaccess ./pkg

serve: build
	cd pkg
	python3 -m http.server 8000

clean:
	cargo clean
	rm -R ./pkg/*

package: build
	zip campagnamica.zip index.html pkg/* pkg/.*
