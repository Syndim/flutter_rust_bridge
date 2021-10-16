#![allow(
    non_camel_case_types,
    clippy::redundant_closure,
    clippy::useless_conversion
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

use flutter_rust_bridge::*;

use crate::api::*;

// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_simple_adder(port: i64, a: i32, b: i32) {
    FLUTTER_RUST_BRIDGE_EXECUTOR.wrap("simple_adder", port, move || {
        let api_a = a.wire2api();
        let api_b = b.wire2api();
        move || simple_adder(api_a, api_b)
    });
}

#[no_mangle]
pub extern "C" fn wire_primitive_types(
    port: i64,
    my_i32: i32,
    my_i64: i64,
    my_f64: f64,
    my_bool: bool,
) {
    FLUTTER_RUST_BRIDGE_EXECUTOR.wrap("primitive_types", port, move || {
        let api_my_i32 = my_i32.wire2api();
        let api_my_i64 = my_i64.wire2api();
        let api_my_f64 = my_f64.wire2api();
        let api_my_bool = my_bool.wire2api();
        move || primitive_types(api_my_i32, api_my_i64, api_my_f64, api_my_bool)
    });
}

#[no_mangle]
pub extern "C" fn wire_handle_string(port: i64, s: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_EXECUTOR.wrap("handle_string", port, move || {
        let api_s = s.wire2api();
        move || handle_string(api_s)
    });
}

#[no_mangle]
pub extern "C" fn wire_handle_vec_u8(port: i64, v: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_EXECUTOR.wrap("handle_vec_u8", port, move || {
        let api_v = v.wire2api();
        move || handle_vec_u8(api_v)
    });
}

#[no_mangle]
pub extern "C" fn wire_handle_zero_copy_result(port: i64, n: i32) {
    FLUTTER_RUST_BRIDGE_EXECUTOR.wrap("handle_zero_copy_result", port, move || {
        let api_n = n.wire2api();
        move || handle_zero_copy_result(api_n)
    });
}

#[no_mangle]
pub extern "C" fn wire_handle_struct(port: i64, arg: *mut wire_MySize, boxed: *mut wire_MySize) {
    FLUTTER_RUST_BRIDGE_EXECUTOR.wrap("handle_struct", port, move || {
        let api_arg = arg.wire2api();
        let api_boxed = boxed.wire2api();
        move || handle_struct(api_arg, api_boxed)
    });
}

#[no_mangle]
pub extern "C" fn wire_handle_newtype(port: i64, arg: *mut wire_NewTypeInt) {
    FLUTTER_RUST_BRIDGE_EXECUTOR.wrap("handle_newtype", port, move || {
        let api_arg = arg.wire2api();
        move || handle_newtype(api_arg)
    });
}

#[no_mangle]
pub extern "C" fn wire_handle_list_of_struct(port: i64, l: *mut wire_list_my_size) {
    FLUTTER_RUST_BRIDGE_EXECUTOR.wrap("handle_list_of_struct", port, move || {
        let api_l = l.wire2api();
        move || handle_list_of_struct(api_l)
    });
}

#[no_mangle]
pub extern "C" fn wire_handle_complex_struct(port: i64, s: *mut wire_MyTreeNode) {
    FLUTTER_RUST_BRIDGE_EXECUTOR.wrap("handle_complex_struct", port, move || {
        let api_s = s.wire2api();
        move || handle_complex_struct(api_s)
    });
}

#[no_mangle]
pub extern "C" fn wire_return_err(port: i64) {
    FLUTTER_RUST_BRIDGE_EXECUTOR.wrap("return_err", port, move || move || return_err());
}

#[no_mangle]
pub extern "C" fn wire_return_panic(port: i64) {
    FLUTTER_RUST_BRIDGE_EXECUTOR.wrap("return_panic", port, move || move || return_panic());
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MySize {
    width: i32,
    height: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_NewTypeInt {
    field0: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_my_size {
    ptr: *mut wire_MySize,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MyTreeNode {
    value_i32: i32,
    value_vec_u8: *mut wire_uint_8_list,
    children: *mut wire_list_my_tree_node,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_my_tree_node {
    ptr: *mut wire_MyTreeNode,
    len: i32,
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_uint_8_list(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_size() -> *mut wire_MySize {
    support::new_leak_box_ptr(wire_MySize::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_my_size() -> *mut wire_MySize {
    support::new_leak_box_ptr(wire_MySize::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_new_type_int() -> *mut wire_NewTypeInt {
    support::new_leak_box_ptr(wire_NewTypeInt::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_list_my_size(len: i32) -> *mut wire_list_my_size {
    let wrap = wire_list_my_size {
        ptr: support::new_leak_vec_ptr(wire_MySize::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_tree_node() -> *mut wire_MyTreeNode {
    support::new_leak_box_ptr(wire_MyTreeNode::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_list_my_tree_node(len: i32) -> *mut wire_list_my_tree_node {
    let wrap = wire_list_my_tree_node {
        ptr: support::new_leak_vec_ptr(wire_MyTreeNode::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}

impl Wire2Api<i64> for i64 {
    fn wire2api(self) -> i64 {
        self
    }
}

impl Wire2Api<f64> for f64 {
    fn wire2api(self) -> f64 {
        self
    }
}

impl Wire2Api<bool> for bool {
    fn wire2api(self) -> bool {
        self
    }
}

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<ZeroCopyBuffer<Vec<u8>>> for *mut wire_uint_8_list {
    fn wire2api(self) -> ZeroCopyBuffer<Vec<u8>> {
        ZeroCopyBuffer(self.wire2api())
    }
}

impl Wire2Api<MySize> for *mut wire_MySize {
    fn wire2api(self) -> MySize {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<MySize> for wire_MySize {
    fn wire2api(self) -> MySize {
        MySize {
            width: self.width.wire2api(),
            height: self.height.wire2api(),
        }
    }
}

impl Wire2Api<Box<MySize>> for *mut wire_MySize {
    fn wire2api(self) -> Box<MySize> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<NewTypeInt> for *mut wire_NewTypeInt {
    fn wire2api(self) -> NewTypeInt {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<NewTypeInt> for wire_NewTypeInt {
    fn wire2api(self) -> NewTypeInt {
        NewTypeInt(self.field0.wire2api())
    }
}

impl Wire2Api<Vec<MySize>> for *mut wire_list_my_size {
    fn wire2api(self) -> Vec<MySize> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(|x| x.wire2api()).collect()
    }
}

impl Wire2Api<MyTreeNode> for *mut wire_MyTreeNode {
    fn wire2api(self) -> MyTreeNode {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<MyTreeNode> for wire_MyTreeNode {
    fn wire2api(self) -> MyTreeNode {
        MyTreeNode {
            value_i32: self.value_i32.wire2api(),
            value_vec_u8: self.value_vec_u8.wire2api(),
            children: self.children.wire2api(),
        }
    }
}

impl Wire2Api<Vec<MyTreeNode>> for *mut wire_list_my_tree_node {
    fn wire2api(self) -> Vec<MyTreeNode> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(|x| x.wire2api()).collect()
    }
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl NewWithNullPtr for wire_MySize {
    fn new_with_null_ptr() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_NewTypeInt {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_MyTreeNode {
    fn new_with_null_ptr() -> Self {
        Self {
            value_i32: Default::default(),
            value_vec_u8: std::ptr::null_mut(),
            children: std::ptr::null_mut(),
        }
    }
}

// Section: impl IntoDart

impl support::IntoDart for MySize {
    fn into_dart(self) -> support::DartCObject {
        vec![self.width.into_dart(), self.height.into_dart()].into_dart()
    }
}

impl support::IntoDart for NewTypeInt {
    fn into_dart(self) -> support::DartCObject {
        vec![self.0.into_dart()].into_dart()
    }
}

impl support::IntoDart for MyTreeNode {
    fn into_dart(self) -> support::DartCObject {
        vec![
            self.value_i32.into_dart(),
            self.value_vec_u8.into_dart(),
            self.children.into_dart(),
        ]
        .into_dart()
    }
}

// Section: executor
support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_EXECUTOR: support::DefaultExecutor = support::DefaultExecutor;
}
