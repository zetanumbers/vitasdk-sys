/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2::common_dialog::*;
#[allow(unused_imports)]
use crate::psp2::kernel::clib::*;
#[allow(unused_imports)]
use crate::psp2::net::net::*;
#[allow(unused_imports)]
use crate::psp2::pspnet_adhocctl::*;
#[allow(unused_imports)]
use crate::psp2::types::*;
use crate::psp2common::net::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub const SCE_NETCHECK_DIALOG_LEAST_HTTP_POOL_SIZE: u32 = 36864;
pub const SCE_NETCHECK_DIALOG_LEAST_SSL_POOL_SIZE: u32 = 98304;
pub const SCE_NETCHECK_DIALOG_INITIAL_AGE_RESTRICTION: i32 = -1;
pub const SCE_NETCHECK_DIALOG_COUNTRY_CODE_LEN: u32 = 2;
pub const SCE_NETCHECK_DIALOG_AGE_RESTRICTION_COUNT_MAX: u32 = 200;
pub mod SceNetCheckDialoErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NETCHECK_DIALOG_ERROR_PARAM: Type = 2148535297;
    pub const SCE_NETCHECK_DIALOG_ERROR_INVALID_MODE: Type = 2148535298;
    pub const SCE_NETCHECK_DIALOG_ERROR_LACK_OF_LIBHTTP_POOL_SIZE: Type = 2148535299;
    pub const SCE_NETCHECK_DIALOG_ERROR_LACK_OF_LIBSSL_POOL_SIZE: Type = 2148535300;
    pub const SCE_NETCHECK_DIALOG_ERROR_LATEST_PATCH_PKG_EXIST: Type = 2148535301;
    pub const SCE_NETCHECK_DIALOG_ERROR_SIGN_OUT: Type = 2148535302;
    pub const SCE_NETCHECK_DIALOG_ERROR_INVALID_PSPADHOC_PARAM: Type = 2148535303;
    pub const SCE_NETCHECK_DIALOG_ERROR_INVALID_TIMEOUT_PARAM: Type = 2148535304;
    pub const SCE_NETCHECK_DIALOG_ERROR_PSN_AGE_RESTRICTION: Type = 2148535305;
}
pub mod SceNetCheckDialogMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NETCHECK_DIALOG_MODE_INVALID: Type = 0;
    pub const SCE_NETCHECK_DIALOG_MODE_ADHOC_CONN: Type = 1;
    pub const SCE_NETCHECK_DIALOG_MODE_PSN: Type = 2;
    pub const SCE_NETCHECK_DIALOG_MODE_PSN_ONLINE: Type = 3;
    pub const SCE_NETCHECK_DIALOG_MODE_PS3_CONNECT: Type = 4;
    pub const SCE_NETCHECK_DIALOG_MODE_PSP_ADHOC_CONN: Type = 5;
    pub const SCE_NETCHECK_DIALOG_MODE_PSP_ADHOC_CREATE: Type = 6;
    pub const SCE_NETCHECK_DIALOG_MODE_PSP_ADHOC_JOIN: Type = 7;
}
pub mod SceNetCheckDialogPS3ConnectAction {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_NETCHECK_DIALOG_PS3_CONNECT_ACTION_ENTER: Type = 0;
    pub const SCE_NETCHECK_DIALOG_PS3_CONNECT_ACTION_LEAVE: Type = 1;
}
#[repr(C)]
pub struct SceNpCommunicationId {
    pub data: [crate::ctypes::c_char; 9usize],
    pub term: crate::ctypes::c_char,
    pub num: SceUChar8,
    pub dummy: crate::ctypes::c_char,
}
#[repr(C)]
pub struct SceNetCheckDialogPS3ConnectParam {
    pub action: SceInt32,
    pub ssid: [crate::ctypes::c_char; 33usize],
    pub wpaKey: [crate::ctypes::c_char; 65usize],
    pub titleId: [crate::ctypes::c_char; 10usize],
}
#[repr(C)]
pub struct SceNetCheckDialogAgeRestriction {
    pub countryCode: [crate::ctypes::c_char; 2usize],
    pub age: SceInt8,
    pub padding: SceInt8,
}
#[repr(C)]
pub struct SceNetCheckDialogParam {
    pub sdkVersion: SceUInt32,
    pub commonParam: SceCommonDialogParam,
    pub mode: SceInt32,
    pub npCommunicationId: SceNpCommunicationId,
    pub ps3ConnectParam: *mut SceNetCheckDialogPS3ConnectParam,
    pub groupName: *mut SceNetAdhocctlGroupName,
    pub timeoutUs: SceUInt32,
    pub defaultAgeRestriction: SceInt8,
    pub padding: [SceInt8; 3usize],
    pub ageRestrictionCount: SceInt32,
    pub ageRestriction: *const SceNetCheckDialogAgeRestriction,
    pub reserved: [SceUInt8; 104usize],
}
#[repr(C)]
pub struct SceNetCheckDialogResult {
    pub result: SceInt32,
    pub psnModeSucceeded: SceBool,
    pub reserved: [SceUInt8; 124usize],
}
#[repr(C)]
pub struct SceNetCheckDialogPS3ConnectInfo {
    pub inaddr: SceNetInAddr,
    pub nickname: [SceUInt8; 128usize],
    pub macAddress: [SceUInt8; 6usize],
    pub reserved: [SceUInt8; 6usize],
}
extern "C" {
    pub fn sceNetCheckDialogInit(param: *mut SceNetCheckDialogParam) -> SceInt32;
}
extern "C" {
    pub fn sceNetCheckDialogGetStatus() -> SceCommonDialogStatus::Type;
}
extern "C" {
    pub fn sceNetCheckDialogAbort() -> SceInt32;
}
extern "C" {
    pub fn sceNetCheckDialogGetResult(result: *mut SceNetCheckDialogResult) -> SceInt32;
}
extern "C" {
    pub fn sceNetCheckDialogGetPS3ConnectInfo(
        info: *mut SceNetCheckDialogPS3ConnectInfo,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNetCheckDialogTerm() -> SceInt32;
}
