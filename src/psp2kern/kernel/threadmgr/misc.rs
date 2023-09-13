/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2common::kernel::threadmgr::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

extern "C" {
    pub fn ksceKernelGetThreadmgrUIDClass(uid: SceUID) -> SceKernelIdListType::Type;
}
extern "C" {
    pub fn ksceKernelGetSystemTimeLow() -> SceUInt32;
}
extern "C" {
    pub fn ksceKernelGetSystemTimeWide() -> SceInt64;
}
extern "C" {
    pub fn ksceKernelGetThreadTLSAddr(
        thid: SceUID,
        key: crate::ctypes::c_int,
    ) -> *mut crate::ctypes::c_void;
}
extern "C" {
    pub fn ksceKernelGetTLSAddr(key: crate::ctypes::c_int) -> *mut crate::ctypes::c_void;
}
extern "C" {
    pub fn ksceKernelSetPermission(value: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelGetProcessId() -> SceUID;
}
extern "C" {
    pub fn ksceKernelGetProcessIdFromTLS() -> SceUID;
}
extern "C" {
    pub fn ksceKernelSetProcessIdToTLS(pid: SceUID) -> SceUID;
}
extern "C" {
    pub fn ksceKernelRunWithStack(
        stack_size: SceSize,
        to_call: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut crate::ctypes::c_void) -> crate::ctypes::c_int,
        >,
        args: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
