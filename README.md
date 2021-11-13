## About

This is a web-app to convert the CSV produced by WordPress's WooCommerce plugin into
a format more amenable to printing labels.

## Usage

Just paste into the textarea the CSV provided by WooCommerce. The corresponding
labels will appear under it. You can then copy the labels, and paste them into the word processor
of your choosing. If there are too many labels and selecting them with the mouse is cumbersome, 
you can click outside the textarea and press Ctrl-A (Cmd-A on macOS) to select everything on 
the page.

## Building

This project is based on the template project provided by https://github.com/yewstack/yew-wasm-pack-minimal.
Anyone wishing to edit this code should familiarise themselves with [Yew](https://yew.rs) and 
[Rust](https://www.rust-lang.org/). Familiarity with the JavaScript ecosystem is recommended.

### 1) Install `Rust`, `wasm-pack` and `rollup`

 * Install rust 1.42: follow the instructions at https://www.rust-lang.org/tools/install; then 
 run the following commands:
    ```
    rustup install 1.42
    rustup default 1.42
    ```
    this ensures that the correct rust version runs by default (there's a bug in a library 
    that prevents using a newer version);
    
 * Install wasm-pack: follow the `installation` link at [`wasm-pack`](https://github.com/rustwasm/wasm-pack);
 * Use `npm` to install `rollup`. 

### 2) Build

A Makefile is provided, so you just need to type `make build` from the project's root directory.
Build artifacts, consisting of a Bundle, are stored under `./pkg`.

### 4) Run Locally

Optionally, if `python3` is available, you can run `make serve` to start a local webserver serving
this app.

Note: it's expected behavior for the browser console to display an error similar to
     
    WebAssembly.instantiateStreaming failed. Assuming this is because your 
    server does not serve wasm with application/wasm MIME type.

Your production webserver should be configured to associate WebAssembly files with the 
`application/wasm` MIME type.

### 5) Deploy

Upload `index.html` and the `pkg/` directory to a webserver of your choosing.