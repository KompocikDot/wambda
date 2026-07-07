use std::fmt::Error;

use wasmtime::Val;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::p1::{self, WasiP1Ctx};

struct State {
    wasi: WasiP1Ctx,
}

pub fn execute_wasm(binary: &[u8], args: &[Val]) -> Result<Vec<Val>, Error> {
    let engine = Engine::default();
    let module = Module::from_binary(&engine, binary).unwrap();

    // 1. Build a WASI context.
    let wasi = WasiCtxBuilder::new().build_p1();

    // 2. Initialize the store with our WASI state
    let mut store = Store::new(&engine, State { wasi });
    let mut linker = Linker::new(&engine);

    // 3. Add all the WASI imports (like `sched_yield`) into the linker
    p1::add_to_linker_sync(&mut linker, |state: &mut State| &mut state.wasi).unwrap();

    // Now it will instantiate successfully because WASI is provided!
    let instance = linker.instantiate(&mut store, &module).unwrap();

    // NOTE FOR GO 1.24+:
    // Go WASI "reactor" modules require you to initialize the runtime once before calling exported functions.
    if let Ok(init) = instance.get_typed_func::<(), ()>(&mut store, "_initialize") {
        init.call(&mut store, ()).unwrap();
    }

    // Call your exported add function
    let add = instance.get_func(&mut store, "fibonnaci").unwrap();

    let fn_type = add.ty(&store);
    let mut output: Vec<Val> = fn_type
        .results()
        .map(|vtype| match vtype {
            wasmtime::ValType::I32 => Val::I32(0),
            wasmtime::ValType::I64 => Val::I64(0),
            wasmtime::ValType::F32 => Val::F32(0),
            wasmtime::ValType::F64 => Val::F64(0),
            _ => panic!("Unsupported Wasm return type!"),
        })
        .collect();

    match add.call(&mut store, args, &mut output) {
        Ok(()) => {}
        Err(e) => eprintln!("{}", e),
    }

    return Ok(output);
}
