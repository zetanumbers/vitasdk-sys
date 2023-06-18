/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::npdrm::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

extern "C" {
    pub fn ksceNpDrmGetRifName(
        name: *mut crate::ctypes::c_char,
        aid: SceUInt64,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNpDrmGetFixedRifName(
        name: *mut crate::ctypes::c_char,
        aid: SceUInt64,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNpDrmGetRifVitaKey(
        license: *const crate::ctypes::c_void,
        klicense: *mut crate::ctypes::c_void,
        flags: *mut crate::ctypes::c_int,
        sku_flags: *mut crate::ctypes::c_int,
        lic_start_time: *mut SceUInt64,
        lic_exp_time: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNpDrmGetRifPspKey(
        license: *const crate::ctypes::c_void,
        klicense: *mut crate::ctypes::c_void,
        flags: *mut crate::ctypes::c_int,
        lic_start_time: *mut SceUInt64,
        lic_exp_time: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNpDrmGetRifInfo(
        license: *const crate::ctypes::c_void,
        license_size: SceSize,
        check_sign: crate::ctypes::c_int,
        content_id: *mut crate::ctypes::c_char,
        account_id: *mut SceUInt64,
        license_version: *mut crate::ctypes::c_int,
        license_flags: *mut crate::ctypes::c_int,
        flags: *mut crate::ctypes::c_int,
        sku_flags: *mut crate::ctypes::c_int,
        lic_start_time: *mut SceInt64,
        lic_exp_time: *mut SceInt64,
        flags2: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNpDrmEbootSigVerify(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_signature: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNpDrmPspEbootVerify(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_signature: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNpDrmPspEbootSigGen(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_sha256: *const crate::ctypes::c_void,
        eboot_signature: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNpDrmEbootSigConvert(
        eboot_pbp_path: *const crate::ctypes::c_char,
        old_eboot_signature: *const crate::ctypes::c_void,
        new_eboot_signature: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNpDrmEbootSigGenPsp(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_sha256: *const crate::ctypes::c_void,
        eboot_signature: *mut crate::ctypes::c_void,
        sw_version: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNpDrmEbootSigGenPs1(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_sha256: *const crate::ctypes::c_void,
        eboot_signature: *mut crate::ctypes::c_void,
        sw_version: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNpDrmEbootSigGenMultiDisc(
        eboot_pbp_path: *const crate::ctypes::c_char,
        sce_discinfo: *const crate::ctypes::c_void,
        eboot_signature: *mut crate::ctypes::c_void,
        sw_version: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
