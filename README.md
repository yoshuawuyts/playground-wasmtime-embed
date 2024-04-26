# playground-wasmtime-embed

A playground showing how to embed Wasmtime in a Rust program. This shows how to
create a Wasm Component interface using WIT, and expose it to the inside of a
Wasmtime sandbox.

Using the WIT definition you can then use `wit-bindgen` or another WIT bindings
generator to create a guest implementation which runs inside of our newly
defined Wasm Component runtime.

## Credit

Thanks to Pat Hickey for providing the step-by-step instructions on how to do
this.

## License

Apache-2.0
