/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

extern "C" {
    pub fn PVRSRVGetMiscInfoKM(info: *mut crate::ctypes::c_void) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceGpuGetRegisterDump(
        dst: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceGpuMmuMapMemory(
        mmuContext: *mut crate::ctypes::c_void,
        vaddr: u32,
        base: *mut crate::ctypes::c_void,
        size: u32,
        flags: u32,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceGpuMmuUnmapMemory(
        mmuContext: *mut crate::ctypes::c_void,
        vaddr: u32,
        size: u32,
    ) -> crate::ctypes::c_int;
}
