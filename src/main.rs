use std::path::PathBuf;

use clap::Parser;
use wasmtime::{component::Component, *};
use wasmtime_wasi::{ResourceTable, WasiView};

wasmtime::component::bindgen!("local:demo/hello");

#[derive(Parser)]
struct Args {
    /// The path to our `.wasm` component
    wasm_path: PathBuf,
    program_input: String,
}

/// The shared context for our component instantiation.
///
/// Each store owns one of these structs. In the linker this maps: names in the
/// component -> functions on the host side.
struct Ctx {
    // Anything that WASI can access is mediated though this. This contains
    // capabilities, preopens, etc.
    wasi: wasmtime_wasi::WasiCtx,
    // NOTE: this might go away eventually
    // We need something which owns the host representation of the resources; we
    // store them in here. Think of it as a `HashMap<i32, Box<dyn Any>>`
    table: wasmtime::component::ResourceTable,
}
impl WasiView for Ctx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        &mut self.wasi
    }
}

fn main() {
    // Get the CLI args
    let args = Args::parse();

    // Setup the engine and linker
    // These pieces can be reused for multiple component instantiations.
    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();
    let component = Component::from_file(&engine, args.wasm_path).unwrap();
    let mut linker: component::Linker<Ctx> = component::Linker::new(&engine);

    // Adds the `wasi:cli/command` world's imports to this linker.
    wasmtime_wasi::command::sync::add_to_linker(&mut linker).unwrap();

    // Instantiate the component!
    // NOTE: `Hello` is created by `component::bindgen!`
    let data = Ctx {
        wasi: wasmtime_wasi::WasiCtxBuilder::new().build(),
        table: wasmtime::component::ResourceTable::new(),
    };
    let mut store: Store<Ctx> = Store::new(&engine, data);
    let (hello, _) = Hello::instantiate(&mut store, &component, &linker).unwrap();

    // Run our component!
    let result = hello
        .local_demo_main()
        .call_run(&mut store, &args.program_input)
        .unwrap();
    println!("{result:?}");
}
