/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::kernel::threadmgr::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

extern "C" {
    pub fn sceKernelCreateLwMutex(
        pWork: *mut SceKernelLwMutexWork,
        pName: *const crate::ctypes::c_char,
        attr: crate::ctypes::c_uint,
        initCount: crate::ctypes::c_int,
        pOptParam: *const SceKernelLwMutexOptParam,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelDeleteLwMutex(pWork: *mut SceKernelLwMutexWork) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelLockLwMutex(
        pWork: *mut SceKernelLwMutexWork,
        lockCount: crate::ctypes::c_int,
        pTimeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelTryLockLwMutex(
        pWork: *mut SceKernelLwMutexWork,
        lockCount: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelUnlockLwMutex(
        pWork: *mut SceKernelLwMutexWork,
        unlockCount: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
