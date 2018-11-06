// Rust test file autogenerated with cargo build (src/build_spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/get_local.wast
#![allow(
    warnings,
    dead_code
)]
use std::panic;
use wabt::wat2wasm;

use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, VmCtx, Export};
use super::_common::{
    spectest_importobject,
    NaNCheck,
};


// Line 3
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (type (;1;) (func (result i64)))
      (type (;2;) (func (result f32)))
      (type (;3;) (func (result f64)))
      (type (;4;) (func (param i32) (result i32)))
      (type (;5;) (func (param i64) (result i64)))
      (type (;6;) (func (param f32) (result f32)))
      (type (;7;) (func (param f64) (result f64)))
      (type (;8;) (func (param i64 f32 f64 i32 i32)))
      (type (;9;) (func (param i64 f32 f64 i32 i32) (result f64)))
      (func (;0;) (type 0) (result i32)
        (local i32)
        get_local 0)
      (func (;1;) (type 1) (result i64)
        (local i64)
        get_local 0)
      (func (;2;) (type 2) (result f32)
        (local f32)
        get_local 0)
      (func (;3;) (type 3) (result f64)
        (local f64)
        get_local 0)
      (func (;4;) (type 4) (param i32) (result i32)
        get_local 0)
      (func (;5;) (type 5) (param i64) (result i64)
        get_local 0)
      (func (;6;) (type 6) (param f32) (result f32)
        get_local 0)
      (func (;7;) (type 7) (param f64) (result f64)
        get_local 0)
      (func (;8;) (type 8) (param i64 f32 f64 i32 i32)
        (local f32 i64 i64 f64)
        get_local 0
        i64.eqz
        drop
        get_local 1
        f32.neg
        drop
        get_local 2
        f64.neg
        drop
        get_local 3
        i32.eqz
        drop
        get_local 4
        i32.eqz
        drop
        get_local 5
        f32.neg
        drop
        get_local 6
        i64.eqz
        drop
        get_local 7
        i64.eqz
        drop
        get_local 8
        f64.neg
        drop)
      (func (;9;) (type 9) (param i64 f32 f64 i32 i32) (result f64)
        (local f32 i64 i64 f64)
        f32.const 0x1.6p+2 (;=5.5;)
        set_local 5
        i64.const 6
        set_local 6
        f64.const 0x1p+3 (;=8;)
        set_local 8
        get_local 0
        f64.convert_u/i64
        get_local 1
        f64.promote/f32
        get_local 2
        get_local 3
        f64.convert_u/i32
        get_local 4
        f64.convert_s/i32
        get_local 5
        f64.promote/f32
        get_local 6
        f64.convert_u/i64
        get_local 7
        f64.convert_u/i64
        get_local 8
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add)
      (export \"type-local-i32\" (func 0))
      (export \"type-local-i64\" (func 1))
      (export \"type-local-f32\" (func 2))
      (export \"type-local-f64\" (func 3))
      (export \"type-param-i32\" (func 4))
      (export \"type-param-i64\" (func 5))
      (export \"type-param-f32\" (func 6))
      (export \"type-param-f64\" (func 7))
      (export \"type-mixed\" (func 8))
      (export \"read\" (func 9)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_1(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 64
fn c1_l64_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c1_l64_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-local-i32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 0 as i32);
}

// Line 65
fn c2_l65_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c2_l65_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-local-i64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 0 as i64);
}

// Line 66
fn c3_l66_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c3_l66_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-local-f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> f32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 0.0 as f32);
}

// Line 67
fn c4_l67_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c4_l67_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-local-f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 0.0 as f64);
}

// Line 69
fn c5_l69_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c5_l69_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-param-i32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2 as i32, &vm_context);
    assert_eq!(result, 2 as i32);
}

// Line 70
fn c6_l70_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c6_l70_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-param-i64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(3 as i64, &vm_context);
    assert_eq!(result, 3 as i64);
}

// Line 71
fn c7_l71_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c7_l71_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-param-f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f32, &VmCtx) -> f32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(4.4 as f32, &vm_context);
    assert_eq!(result, 4.4 as f32);
}

// Line 72
fn c8_l72_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c8_l72_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-param-f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f64, &VmCtx) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(5.5 as f64, &vm_context);
    assert_eq!(result, 5.5 as f64);
}

// Line 75
fn c9_l75_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c9_l75_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-mixed") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, f32, f64, i32, i32, &VmCtx) = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(1 as i64, 2.2 as f32, 3.3 as f64, 4 as i32, 5 as i32, &vm_context);
    assert_eq!(result, ());
}

// Line 81
fn c10_l81_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c10_l81_action_invoke");
    let func_index = match result_object.module.info.exports.get("read") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, f32, f64, i32, i32, &VmCtx) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(1 as i64, 2.0 as f32, 3.3 as f64, 4 as i32, 5 as i32, &vm_context);
    assert_eq!(result, 34.8 as f64);
}

// Line 91
#[test]
fn c11_l91_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 126, 3, 2, 1, 0, 10, 8, 1, 6, 1, 1, 127, 32, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 95
#[test]
fn c12_l95_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 9, 1, 7, 1, 1, 125, 32, 0, 69, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 99
#[test]
fn c13_l99_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 11, 1, 9, 2, 1, 124, 1, 126, 32, 1, 154, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 107
#[test]
fn c14_l107_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 1, 127, 1, 126, 3, 2, 1, 0, 10, 6, 1, 4, 0, 32, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 111
#[test]
fn c15_l111_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 125, 0, 3, 2, 1, 0, 10, 7, 1, 5, 0, 32, 0, 69, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 115
#[test]
fn c16_l115_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 2, 124, 126, 0, 3, 2, 1, 0, 10, 7, 1, 5, 0, 32, 1, 154, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 123
#[test]
fn c17_l123_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 10, 1, 8, 2, 1, 127, 1, 126, 32, 3, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 127
#[test]
fn c18_l127_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 13, 1, 11, 2, 1, 127, 1, 126, 32, 247, 164, 234, 6, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 132
#[test]
fn c19_l132_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 2, 127, 126, 0, 3, 2, 1, 0, 10, 6, 1, 4, 0, 32, 2, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 136
#[test]
fn c20_l136_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 14, 1, 12, 2, 1, 127, 1, 126, 32, 247, 242, 206, 212, 2, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 141
#[test]
fn c21_l141_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 127, 0, 3, 2, 1, 0, 10, 10, 1, 8, 2, 1, 127, 1, 126, 32, 3, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 145
#[test]
fn c22_l145_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 126, 0, 3, 2, 1, 0, 10, 13, 1, 11, 2, 1, 127, 1, 126, 32, 247, 168, 153, 102, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_1(&result_object, &vm_context);
    c1_l64_action_invoke(&result_object, &vm_context);
    c2_l65_action_invoke(&result_object, &vm_context);
    c3_l66_action_invoke(&result_object, &vm_context);
    c4_l67_action_invoke(&result_object, &vm_context);
    c5_l69_action_invoke(&result_object, &vm_context);
    c6_l70_action_invoke(&result_object, &vm_context);
    c7_l71_action_invoke(&result_object, &vm_context);
    c8_l72_action_invoke(&result_object, &vm_context);
    c9_l75_action_invoke(&result_object, &vm_context);
    c10_l81_action_invoke(&result_object, &vm_context);
}
