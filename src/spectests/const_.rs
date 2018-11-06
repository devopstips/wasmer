// Rust test file autogenerated with cargo build (src/build_spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/const_.wast
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


// Line 5
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        i32.const -1
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_1(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 6

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_1(&result_object, &vm_context);
}
fn create_module_2() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        i32.const -2147483648
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_2(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 8
#[test]
fn c2_l8_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 48, 120, 49, 48, 48, 48, 48, 48, 48, 48, 48, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 12
#[test]
fn c3_l12_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 45, 48, 120, 56, 48, 48, 48, 48, 48, 48, 49, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 16

#[test]
fn test_module_2() {
    let result_object = create_module_2();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_2(&result_object, &vm_context);
}
fn create_module_3() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        i32.const -1
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_3(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 17

#[test]
fn test_module_3() {
    let result_object = create_module_3();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_3(&result_object, &vm_context);
}
fn create_module_4() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        i32.const -2147483648
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_4(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 19
#[test]
fn c6_l19_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 52, 50, 57, 52, 57, 54, 55, 50, 57, 54, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 23
#[test]
fn c7_l23_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 45, 50, 49, 52, 55, 52, 56, 51, 54, 52, 57, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 27

#[test]
fn test_module_4() {
    let result_object = create_module_4();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_4(&result_object, &vm_context);
}
fn create_module_5() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        i64.const -1
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_5(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 28

#[test]
fn test_module_5() {
    let result_object = create_module_5();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_5(&result_object, &vm_context);
}
fn create_module_6() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        i64.const -9223372036854775808
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_6(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 30
#[test]
fn c10_l30_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 48, 120, 49, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 34
#[test]
fn c11_l34_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 45, 48, 120, 56, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 49, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 38

#[test]
fn test_module_6() {
    let result_object = create_module_6();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_6(&result_object, &vm_context);
}
fn create_module_7() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        i64.const -1
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_7(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 39

#[test]
fn test_module_7() {
    let result_object = create_module_7();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_7(&result_object, &vm_context);
}
fn create_module_8() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        i64.const -9223372036854775808
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_8(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 41
#[test]
fn c14_l41_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 49, 56, 52, 52, 54, 55, 52, 52, 48, 55, 51, 55, 48, 57, 53, 53, 49, 54, 49, 54, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 45
#[test]
fn c15_l45_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 45, 57, 50, 50, 51, 51, 55, 50, 48, 51, 54, 56, 53, 52, 55, 55, 53, 56, 48, 57, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 49

#[test]
fn test_module_8() {
    let result_object = create_module_8();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_8(&result_object, &vm_context);
}
fn create_module_9() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f32.const 0x1p+127 (;=1.70141e+38;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_9(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 50

#[test]
fn test_module_9() {
    let result_object = create_module_9();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_9(&result_object, &vm_context);
}
fn create_module_10() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f32.const -0x1p+127 (;=-1.70141e+38;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_10(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 51

#[test]
fn test_module_10() {
    let result_object = create_module_10();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_10(&result_object, &vm_context);
}
fn create_module_11() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f32.const 0x1.fffffep+127 (;=3.40282e+38;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_11(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 52

#[test]
fn test_module_11() {
    let result_object = create_module_11();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_11(&result_object, &vm_context);
}
fn create_module_12() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f32.const -0x1.fffffep+127 (;=-3.40282e+38;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_12(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 53

#[test]
fn test_module_12() {
    let result_object = create_module_12();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_12(&result_object, &vm_context);
}
fn create_module_13() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f32.const 0x1.fffffep+127 (;=3.40282e+38;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_13(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 54

#[test]
fn test_module_13() {
    let result_object = create_module_13();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_13(&result_object, &vm_context);
}
fn create_module_14() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f32.const -0x1.fffffep+127 (;=-3.40282e+38;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_14(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 56
#[test]
fn c22_l56_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 51, 50, 46, 99, 111, 110, 115, 116, 32, 48, 120, 49, 112, 49, 50, 56, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 60
#[test]
fn c23_l60_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 51, 50, 46, 99, 111, 110, 115, 116, 32, 45, 48, 120, 49, 112, 49, 50, 56, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 64
#[test]
fn c24_l64_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 51, 50, 46, 99, 111, 110, 115, 116, 32, 48, 120, 49, 46, 102, 102, 102, 102, 102, 102, 112, 49, 50, 55, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 68
#[test]
fn c25_l68_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 51, 50, 46, 99, 111, 110, 115, 116, 32, 45, 48, 120, 49, 46, 102, 102, 102, 102, 102, 102, 112, 49, 50, 55, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 72

#[test]
fn test_module_14() {
    let result_object = create_module_14();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_14(&result_object, &vm_context);
}
fn create_module_15() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f32.const 0x1.2ced32p+126 (;=1e+38;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_15(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 73

#[test]
fn test_module_15() {
    let result_object = create_module_15();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_15(&result_object, &vm_context);
}
fn create_module_16() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f32.const -0x1.2ced32p+126 (;=-1e+38;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_16(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 75
#[test]
fn c28_l75_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 51, 50, 46, 99, 111, 110, 115, 116, 32, 49, 101, 51, 57, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 79
#[test]
fn c29_l79_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 51, 50, 46, 99, 111, 110, 115, 116, 32, 45, 49, 101, 51, 57, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 83

#[test]
fn test_module_16() {
    let result_object = create_module_16();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_16(&result_object, &vm_context);
}
fn create_module_17() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f32.const 0x1.fffffep+127 (;=3.40282e+38;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_17(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 84

#[test]
fn test_module_17() {
    let result_object = create_module_17();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_17(&result_object, &vm_context);
}
fn create_module_18() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f32.const -0x1.fffffep+127 (;=-3.40282e+38;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_18(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 86
#[test]
fn c32_l86_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 51, 50, 46, 99, 111, 110, 115, 116, 32, 51, 52, 48, 50, 56, 50, 51, 53, 54, 55, 55, 57, 55, 51, 51, 54, 54, 49, 54, 51, 55, 53, 51, 57, 51, 57, 53, 52, 53, 56, 49, 52, 50, 53, 54, 56, 52, 52, 56, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 90
#[test]
fn c33_l90_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 51, 50, 46, 99, 111, 110, 115, 116, 32, 45, 51, 52, 48, 50, 56, 50, 51, 53, 54, 55, 55, 57, 55, 51, 51, 54, 54, 49, 54, 51, 55, 53, 51, 57, 51, 57, 53, 52, 53, 56, 49, 52, 50, 53, 54, 56, 52, 52, 56, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 94

#[test]
fn test_module_18() {
    let result_object = create_module_18();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_18(&result_object, &vm_context);
}
fn create_module_19() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f64.const 0x1p+1023 (;=8.98847e+307;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_19(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 95

#[test]
fn test_module_19() {
    let result_object = create_module_19();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_19(&result_object, &vm_context);
}
fn create_module_20() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f64.const -0x1p+1023 (;=-8.98847e+307;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_20(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 96

#[test]
fn test_module_20() {
    let result_object = create_module_20();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_20(&result_object, &vm_context);
}
fn create_module_21() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f64.const 0x1.fffffffffffffp+1023 (;=1.79769e+308;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_21(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 97

#[test]
fn test_module_21() {
    let result_object = create_module_21();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_21(&result_object, &vm_context);
}
fn create_module_22() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f64.const -0x1.fffffffffffffp+1023 (;=-1.79769e+308;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_22(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 98

#[test]
fn test_module_22() {
    let result_object = create_module_22();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_22(&result_object, &vm_context);
}
fn create_module_23() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f64.const 0x1.fffffffffffffp+1023 (;=1.79769e+308;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_23(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 99

#[test]
fn test_module_23() {
    let result_object = create_module_23();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_23(&result_object, &vm_context);
}
fn create_module_24() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f64.const -0x1.fffffffffffffp+1023 (;=-1.79769e+308;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_24(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 101
#[test]
fn c40_l101_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 54, 52, 46, 99, 111, 110, 115, 116, 32, 48, 120, 49, 112, 49, 48, 50, 52, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 105
#[test]
fn c41_l105_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 54, 52, 46, 99, 111, 110, 115, 116, 32, 45, 48, 120, 49, 112, 49, 48, 50, 52, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 109
#[test]
fn c42_l109_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 54, 52, 46, 99, 111, 110, 115, 116, 32, 48, 120, 49, 46, 102, 102, 102, 102, 102, 102, 102, 102, 102, 102, 102, 102, 102, 56, 112, 49, 48, 50, 51, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 113
#[test]
fn c43_l113_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 54, 52, 46, 99, 111, 110, 115, 116, 32, 45, 48, 120, 49, 46, 102, 102, 102, 102, 102, 102, 102, 102, 102, 102, 102, 102, 102, 56, 112, 49, 48, 50, 51, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 117

#[test]
fn test_module_24() {
    let result_object = create_module_24();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_24(&result_object, &vm_context);
}
fn create_module_25() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f64.const 0x1.1ccf385ebc8ap+1023 (;=1e+308;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_25(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 118

#[test]
fn test_module_25() {
    let result_object = create_module_25();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_25(&result_object, &vm_context);
}
fn create_module_26() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f64.const -0x1.1ccf385ebc8ap+1023 (;=-1e+308;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_26(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 120
#[test]
fn c46_l120_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 54, 52, 46, 99, 111, 110, 115, 116, 32, 49, 101, 51, 48, 57, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 124
#[test]
fn c47_l124_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 54, 52, 46, 99, 111, 110, 115, 116, 32, 45, 49, 101, 51, 48, 57, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 128

#[test]
fn test_module_26() {
    let result_object = create_module_26();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_26(&result_object, &vm_context);
}
fn create_module_27() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f64.const 0x1.fffffffffffffp+1023 (;=1.79769e+308;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_27(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 129

#[test]
fn test_module_27() {
    let result_object = create_module_27();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_27(&result_object, &vm_context);
}
fn create_module_28() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0)
        f64.const -0x1.fffffffffffffp+1023 (;=-1.79769e+308;)
        drop))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_28(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 131
#[test]
fn c50_l131_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 54, 52, 46, 99, 111, 110, 115, 116, 32, 50, 54, 57, 54, 53, 51, 57, 55, 48, 50, 50, 57, 51, 52, 55, 51, 53, 54, 50, 50, 49, 55, 57, 49, 49, 51, 53, 53, 57, 55, 53, 53, 54, 53, 51, 53, 49, 57, 55, 49, 48, 53, 56, 53, 49, 50, 56, 56, 55, 54, 55, 52, 57, 52, 56, 57, 56, 51, 55, 54, 50, 49, 53, 50, 48, 52, 55, 51, 53, 56, 57, 49, 49, 55, 48, 48, 52, 50, 56, 48, 56, 49, 52, 48, 56, 56, 52, 51, 51, 55, 57, 52, 57, 49, 53, 48, 51, 49, 55, 50, 53, 55, 51, 49, 48, 54, 56, 56, 52, 51, 48, 50, 55, 49, 53, 55, 51, 54, 57, 54, 51, 53, 49, 52, 56, 49, 57, 57, 48, 51, 51, 52, 49, 57, 54, 50, 55, 52, 49, 53, 50, 55, 48, 49, 51, 50, 48, 48, 53, 53, 51, 48, 54, 50, 55, 53, 52, 55, 57, 48, 55, 52, 56, 54, 53, 56, 54, 52, 56, 50, 54, 57, 50, 51, 49, 49, 52, 51, 54, 56, 50, 51, 53, 49, 51, 53, 53, 56, 51, 57, 57, 51, 52, 49, 54, 49, 49, 51, 56, 48, 50, 55, 54, 50, 54, 56, 50, 55, 48, 48, 57, 49, 51, 52, 53, 54, 56, 55, 52, 56, 53, 53, 51, 53, 52, 56, 51, 52, 52, 50, 50, 50, 52, 56, 55, 49, 50, 56, 51, 56, 57, 57, 56, 49, 56, 53, 48, 50, 50, 52, 49, 50, 49, 57, 54, 55, 51, 57, 51, 48, 54, 50, 49, 55, 48, 56, 52, 55, 53, 51, 49, 48, 55, 50, 54, 53, 55, 55, 49, 51, 55, 56, 57, 52, 57, 56, 50, 49, 56, 55, 53, 54, 48, 54, 48, 51, 57, 50, 55, 54, 49, 56, 55, 50, 56, 55, 53, 53, 50, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 135
#[test]
fn c51_l135_assert_malformed() {
    let wasm_binary = [40, 102, 117, 110, 99, 32, 40, 102, 54, 52, 46, 99, 111, 110, 115, 116, 32, 45, 50, 54, 57, 54, 53, 51, 57, 55, 48, 50, 50, 57, 51, 52, 55, 51, 53, 54, 50, 50, 49, 55, 57, 49, 49, 51, 53, 53, 57, 55, 53, 53, 54, 53, 51, 53, 49, 57, 55, 49, 48, 53, 56, 53, 49, 50, 56, 56, 55, 54, 55, 52, 57, 52, 56, 57, 56, 51, 55, 54, 50, 49, 53, 50, 48, 52, 55, 51, 53, 56, 57, 49, 49, 55, 48, 48, 52, 50, 56, 48, 56, 49, 52, 48, 56, 56, 52, 51, 51, 55, 57, 52, 57, 49, 53, 48, 51, 49, 55, 50, 53, 55, 51, 49, 48, 54, 56, 56, 52, 51, 48, 50, 55, 49, 53, 55, 51, 54, 57, 54, 51, 53, 49, 52, 56, 49, 57, 57, 48, 51, 51, 52, 49, 57, 54, 50, 55, 52, 49, 53, 50, 55, 48, 49, 51, 50, 48, 48, 53, 53, 51, 48, 54, 50, 55, 53, 52, 55, 57, 48, 55, 52, 56, 54, 53, 56, 54, 52, 56, 50, 54, 57, 50, 51, 49, 49, 52, 51, 54, 56, 50, 51, 53, 49, 51, 53, 53, 56, 51, 57, 57, 51, 52, 49, 54, 49, 49, 51, 56, 48, 50, 55, 54, 50, 54, 56, 50, 55, 48, 48, 57, 49, 51, 52, 53, 54, 56, 55, 52, 56, 53, 53, 51, 53, 52, 56, 51, 52, 52, 50, 50, 50, 52, 56, 55, 49, 50, 56, 51, 56, 57, 57, 56, 49, 56, 53, 48, 50, 50, 52, 49, 50, 49, 57, 54, 55, 51, 57, 51, 48, 54, 50, 49, 55, 48, 56, 52, 55, 53, 51, 49, 48, 55, 50, 54, 53, 55, 55, 49, 51, 55, 56, 57, 52, 57, 56, 50, 49, 56, 55, 53, 54, 48, 54, 48, 51, 57, 50, 55, 54, 49, 56, 55, 50, 56, 55, 53, 53, 50, 41, 32, 100, 114, 111, 112, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

#[test]
fn test_module_28() {
    let result_object = create_module_28();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_28(&result_object, &vm_context);
}
