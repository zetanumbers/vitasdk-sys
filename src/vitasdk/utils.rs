/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

extern "C" {
    pub fn vitasdk_get_tls_data(thid: SceUID) -> *mut crate::ctypes::c_void;
}
extern "C" {
    pub fn vitasdk_get_pthread_data(thid: SceUID) -> *mut crate::ctypes::c_void;
}
extern "C" {
    pub fn vitasdk_delete_thread_reent(thid: SceUID) -> crate::ctypes::c_int;
}
