use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::p1::{self, WasiP1Ctx};

struct State {
    wasi: WasiP1Ctx,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::default();
    let module = Module::from_file(
        &engine,
        "/Users/kacper/Projects/faasm/test_functions/go/main.wasm",
    )?;

    // 1. Build a WASI context.
    let wasi = WasiCtxBuilder::new().build_p1();

    // 2. Initialize the store with our WASI state
    let mut store = Store::new(&engine, State { wasi });
    let mut linker = Linker::new(&engine);

    // 3. Add all the WASI imports (like `sched_yield`) into the linker
    p1::add_to_linker_sync(&mut linker, |state: &mut State| &mut state.wasi)?;

    // Now it will instantiate successfully because WASI is provided!
    let instance = linker.instantiate(&mut store, &module)?;

    // NOTE FOR GO 1.24+:
    // Go WASI "reactor" modules require you to initialize the runtime once before calling exported functions.
    if let Ok(init) = instance.get_typed_func::<(), ()>(&mut store, "_initialize") {
        init.call(&mut store, ())?;
    }

    // Call your exported add function
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    let result = add.call(&mut store, (2, 3))?;

    println!("Result from Go Wasm: {}", result);

    Ok(())
}
