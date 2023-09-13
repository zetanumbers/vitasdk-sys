/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::defs::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSharedFbInfo {
    pub fb_base: *mut crate::ctypes::c_void,
    pub fb_size: crate::ctypes::c_int,
    pub fb_base2: *mut crate::ctypes::c_void,
    pub unk0: [crate::ctypes::c_int; 6usize],
    pub stride: crate::ctypes::c_int,
    pub width: crate::ctypes::c_int,
    pub height: crate::ctypes::c_int,
    pub unk1: crate::ctypes::c_int,
    pub index: crate::ctypes::c_int,
    pub unk2: [crate::ctypes::c_int; 4usize],
    pub vsync: crate::ctypes::c_int,
    pub unk3: [crate::ctypes::c_int; 3usize],
}
extern "C" {
    pub fn _sceSharedFbOpen(index: crate::ctypes::c_int, sysver: crate::ctypes::c_int) -> SceUID;
}
extern "C" {
    pub fn sceSharedFbClose(fb_id: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSharedFbBegin(fb_id: SceUID, info: *mut SceSharedFbInfo) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSharedFbEnd(fb_id: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSharedFbGetInfo(fb_id: SceUID, info: *mut SceSharedFbInfo) -> crate::ctypes::c_int;
}
