build:
	wasm-pack build --target web

serve:
	python3 -m http.server 8080

clean:
	cargo clean
	rm -rf pkg