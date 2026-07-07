use serde::Deserialize;
use wasmtime::Val;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum WasmtimeVal {
    I32(i32),
    // I64(i64),
    // F32(u32),
    // F64(u64),
}

pub fn from_json(v: WasmtimeVal) -> Val {
    return match v {
        WasmtimeVal::I32(v) => Val::I32(v),
        // WasmtimeVal::I64(v) => Val::I64(v),
        // WasmtimeVal::F32(v) => Val::F32(v),
        // WasmtimeVal::F64(v) => Val::F64(v),
    };
}

pub fn to_json(v: Val) -> WasmtimeVal {
    return match v {
        Val::I32(v) => WasmtimeVal::I32(v),
        // Val::I64(v) => WasmtimeVal::I64(v),
        // Val::F32(v) => WasmtimeVal::F32(v),
        // Val::F64(v) => WasmtimeVal::F64(v),
        _ => panic!("Unsupported type {:?}", v),
    };
}
