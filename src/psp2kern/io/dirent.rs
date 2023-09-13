/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2common::kernel::iofilemgr::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

extern "C" {
    pub fn ksceIoDopen(dirname: *const crate::ctypes::c_char) -> SceUID;
}
extern "C" {
    pub fn ksceIoDread(fd: SceUID, dir: *mut SceIoDirent) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceIoDclose(fd: SceUID) -> crate::ctypes::c_int;
}
