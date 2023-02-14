
run: rust-bootstrap-component.wasm
	cargo run

wasm-tools:
	cargo install wasm-tools --git https://github.com/bytecodealliance/wasm-tools --rev 89c3be6

rust-bootstrap-component.wasm: rust-bootstrap.wasm wasm-tools
	wasm-tools component new $< -o $@ --adapt wasi_snapshot_preview1.wasm

DEL = rm -rf
clean:
	$(DEL) rust-bootstrap-*.wasm