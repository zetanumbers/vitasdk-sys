/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub mod SceSysmoduleErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SYSMODULE_LOADED: Type = 0;
    pub const SCE_SYSMODULE_ERROR_INVALID_VALUE: Type = 2153385984;
    pub const SCE_SYSMODULE_ERROR_UNLOADED: Type = 2153385985;
    pub const SCE_SYSMODULE_ERROR_FATAL: Type = 2153386239;
}
pub mod SceSysmoduleModuleId {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SYSMODULE_INVALID: Type = 0;
    pub const SCE_SYSMODULE_NET: Type = 1;
    pub const SCE_SYSMODULE_HTTP: Type = 2;
    pub const SCE_SYSMODULE_SSL: Type = 3;
    pub const SCE_SYSMODULE_HTTPS: Type = 4;
    pub const SCE_SYSMODULE_PERF: Type = 5;
    pub const SCE_SYSMODULE_FIBER: Type = 6;
    pub const SCE_SYSMODULE_ULT: Type = 7;
    pub const SCE_SYSMODULE_DBG: Type = 8;
    pub const SCE_SYSMODULE_RAZOR_CAPTURE: Type = 9;
    pub const SCE_SYSMODULE_RAZOR_HUD: Type = 10;
    pub const SCE_SYSMODULE_NGS: Type = 11;
    pub const SCE_SYSMODULE_SULPHA: Type = 12;
    pub const SCE_SYSMODULE_SAS: Type = 13;
    pub const SCE_SYSMODULE_PGF: Type = 14;
    pub const SCE_SYSMODULE_APPUTIL: Type = 15;
    pub const SCE_SYSMODULE_FIOS2: Type = 16;
    pub const SCE_SYSMODULE_IME: Type = 17;
    pub const SCE_SYSMODULE_NP_BASIC: Type = 18;
    pub const SCE_SYSMODULE_SYSTEM_GESTURE: Type = 19;
    pub const SCE_SYSMODULE_LOCATION: Type = 20;
    pub const SCE_SYSMODULE_NP: Type = 21;
    pub const SCE_SYSMODULE_PHOTO_EXPORT: Type = 22;
    pub const SCE_SYSMODULE_XML: Type = 23;
    pub const SCE_SYSMODULE_NP_COMMERCE2: Type = 24;
    pub const SCE_SYSMODULE_NP_UTILITY: Type = 25;
    pub const SCE_SYSMODULE_VOICE: Type = 26;
    pub const SCE_SYSMODULE_VOICEQOS: Type = 27;
    pub const SCE_SYSMODULE_NP_MATCHING2: Type = 28;
    pub const SCE_SYSMODULE_SCREEN_SHOT: Type = 29;
    pub const SCE_SYSMODULE_NP_SCORE_RANKING: Type = 30;
    pub const SCE_SYSMODULE_SQLITE: Type = 31;
    pub const SCE_SYSMODULE_TRIGGER_UTIL: Type = 32;
    pub const SCE_SYSMODULE_RUDP: Type = 33;
    pub const SCE_SYSMODULE_CODECENGINE_PERF: Type = 34;
    pub const SCE_SYSMODULE_LIVEAREA: Type = 35;
    pub const SCE_SYSMODULE_NP_ACTIVITY: Type = 36;
    pub const SCE_SYSMODULE_NP_TROPHY: Type = 37;
    pub const SCE_SYSMODULE_NP_MESSAGE: Type = 38;
    pub const SCE_SYSMODULE_SHUTTER_SOUND: Type = 39;
    pub const SCE_SYSMODULE_CLIPBOARD: Type = 40;
    pub const SCE_SYSMODULE_NP_PARTY: Type = 41;
    pub const SCE_SYSMODULE_NET_ADHOC_MATCHING: Type = 42;
    pub const SCE_SYSMODULE_NEAR_UTIL: Type = 43;
    pub const SCE_SYSMODULE_NP_TUS: Type = 44;
    pub const SCE_SYSMODULE_MP4: Type = 45;
    pub const SCE_SYSMODULE_AACENC: Type = 46;
    pub const SCE_SYSMODULE_HANDWRITING: Type = 47;
    pub const SCE_SYSMODULE_ATRAC: Type = 48;
    pub const SCE_SYSMODULE_NP_SNS_FACEBOOK: Type = 49;
    pub const SCE_SYSMODULE_VIDEO_EXPORT: Type = 50;
    pub const SCE_SYSMODULE_NOTIFICATION_UTIL: Type = 51;
    pub const SCE_SYSMODULE_BG_APP_UTIL: Type = 52;
    pub const SCE_SYSMODULE_INCOMING_DIALOG: Type = 53;
    pub const SCE_SYSMODULE_IPMI: Type = 54;
    pub const SCE_SYSMODULE_AUDIOCODEC: Type = 55;
    pub const SCE_SYSMODULE_FACE: Type = 56;
    pub const SCE_SYSMODULE_SMART: Type = 57;
    pub const SCE_SYSMODULE_MARLIN: Type = 58;
    pub const SCE_SYSMODULE_MARLIN_DOWNLOADER: Type = 59;
    pub const SCE_SYSMODULE_MARLIN_APP_LIB: Type = 60;
    pub const SCE_SYSMODULE_TELEPHONY_UTIL: Type = 61;
    pub const SCE_SYSMODULE_SHACCCG: Type = 62;
    pub const SCE_SYSMODULE_MONO_BRIDGE: Type = 63;
    pub const SCE_SYSMODULE_MONO: Type = 64;
    pub const SCE_SYSMODULE_PSM: Type = 65;
    pub const SCE_SYSMODULE_PSM_DEVAGENT: Type = 66;
    pub const SCE_SYSMODULE_PSPNET_ADHOC: Type = 67;
    pub const SCE_SYSMODULE_DTCP_IP: Type = 68;
    pub const SCE_SYSMODULE_VIDEO_SEARCH_EMPR: Type = 69;
    pub const SCE_SYSMODULE_NP_SIGNALING: Type = 70;
    pub const SCE_SYSMODULE_BEISOBMF: Type = 71;
    pub const SCE_SYSMODULE_BEMP2SYS: Type = 72;
    pub const SCE_SYSMODULE_MUSIC_EXPORT: Type = 73;
    pub const SCE_SYSMODULE_NEAR_DIALOG_UTIL: Type = 74;
    pub const SCE_SYSMODULE_LOCATION_EXTENSION: Type = 75;
    pub const SCE_SYSMODULE_AVPLAYER: Type = 76;
    pub const SCE_SYSMODULE_GAME_UPDATE: Type = 77;
    pub const SCE_SYSMODULE_MAIL_API: Type = 78;
    pub const SCE_SYSMODULE_TELEPORT_CLIENT: Type = 79;
    pub const SCE_SYSMODULE_TELEPORT_SERVER: Type = 80;
    pub const SCE_SYSMODULE_MP4_RECORDER: Type = 81;
    pub const SCE_SYSMODULE_APPUTIL_EXT: Type = 82;
    pub const SCE_SYSMODULE_NP_WEBAPI: Type = 83;
    pub const SCE_SYSMODULE_AVCDEC: Type = 84;
    pub const SCE_SYSMODULE_JSON: Type = 85;
}
pub mod SceSysmoduleInternalModuleId {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SYSMODULE_INTERNAL_JPEG_ENC_ARM: Type = 2147483649;
    pub const SCE_SYSMODULE_INTERNAL_AUDIOCODEC: Type = 2147483650;
    pub const SCE_SYSMODULE_INTERNAL_JPEG_ARM: Type = 2147483651;
    pub const SCE_SYSMODULE_INTERNAL_G729: Type = 2147483652;
    pub const SCE_SYSMODULE_INTERNAL_BXCE: Type = 2147483653;
    pub const SCE_SYSMODULE_INTERNAL_INI_FILE_PROCESSOR: Type = 2147483654;
    pub const SCE_SYSMODULE_INTERNAL_NP_ACTIVITY_NET: Type = 2147483655;
    pub const SCE_SYSMODULE_INTERNAL_PAF: Type = 2147483656;
    pub const SCE_SYSMODULE_INTERNAL_SQLITE_VSH: Type = 2147483657;
    pub const SCE_SYSMODULE_INTERNAL_DBUTIL: Type = 2147483658;
    pub const SCE_SYSMODULE_INTERNAL_ACTIVITY_DB: Type = 2147483659;
    pub const SCE_SYSMODULE_INTERNAL_COMMON_GUI_DIALOG: Type = 2147483660;
    pub const SCE_SYSMODULE_INTERNAL_STORE_CHECKOUT: Type = 2147483661;
    pub const SCE_SYSMODULE_INTERNAL_IME_DIALOG: Type = 2147483662;
    pub const SCE_SYSMODULE_INTERNAL_PHOTO_IMPORT_DIALOG: Type = 2147483663;
    pub const SCE_SYSMODULE_INTERNAL_PHOTO_REVIEW_DIALOG: Type = 2147483664;
    pub const SCE_SYSMODULE_INTERNAL_CHECKOUT_DIALOG: Type = 2147483665;
    pub const SCE_SYSMODULE_INTERNAL_COMMON_DIALOG_MAIN: Type = 2147483666;
    pub const SCE_SYSMODULE_INTERNAL_MSG_DIALOG: Type = 2147483667;
    pub const SCE_SYSMODULE_INTERNAL_NET_CHECK_DIALOG: Type = 2147483668;
    pub const SCE_SYSMODULE_INTERNAL_SAVEDATA_DIALOG: Type = 2147483669;
    pub const SCE_SYSMODULE_INTERNAL_NP_MESSAGE_DIALOG: Type = 2147483670;
    pub const SCE_SYSMODULE_INTERNAL_TROPHY_SETUP_DIALOG: Type = 2147483671;
    pub const SCE_SYSMODULE_INTERNAL_FRIEND_LIST_DIALOG: Type = 2147483672;
    pub const SCE_SYSMODULE_INTERNAL_NEAR_PROFILE: Type = 2147483674;
    pub const SCE_SYSMODULE_INTERNAL_NP_FRIEND_PRIVACY_LEVEL: Type = 2147483675;
    pub const SCE_SYSMODULE_INTERNAL_NP_COMMERCE2: Type = 2147483677;
    pub const SCE_SYSMODULE_INTERNAL_NP_KDC: Type = 2147483678;
    pub const SCE_SYSMODULE_INTERNAL_MUSIC_EXPORT: Type = 2147483679;
    pub const SCE_SYSMODULE_INTERNAL_VIDEO_EXPORT: Type = 2147483680;
    pub const SCE_SYSMODULE_INTERNAL_NP_MESSAGE_DIALOG_IMPL: Type = 2147483681;
    pub const SCE_SYSMODULE_INTERNAL_NP_MESSAGE_CONTACTS: Type = 2147483682;
    pub const SCE_SYSMODULE_INTERNAL_DB_RECOVERY_UTILITY: Type = 2147483683;
    pub const SCE_SYSMODULE_INTERNAL_PROMOTER_UTIL: Type = 2147483684;
    pub const SCE_SYSMODULE_INTERNAL_PARTY_MEMBER_LIST: Type = 2147483686;
    pub const SCE_SYSMODULE_INTERNAL_ULT: Type = 2147483685;
    pub const SCE_SYSMODULE_INTERNAL_DRM_PSM_KDC: Type = 2147483687;
    pub const SCE_SYSMODULE_INTERNAL_LOCATION_INTERNAL: Type = 2147483688;
    pub const SCE_SYSMODULE_INTERNAL_LOCATION_FACTORY: Type = 2147483689;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSysmoduleOpt {
    pub flags: crate::ctypes::c_int,
    pub result: *mut crate::ctypes::c_int,
    pub unused: [crate::ctypes::c_int; 2usize],
}
extern "C" {
    pub fn sceSysmoduleLoadModule(id: SceSysmoduleModuleId::Type) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSysmoduleUnloadModule(id: SceSysmoduleModuleId::Type) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSysmoduleIsLoaded(id: SceSysmoduleModuleId::Type) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSysmoduleLoadModuleInternal(
        id: SceSysmoduleInternalModuleId::Type,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSysmoduleUnloadModuleInternal(
        id: SceSysmoduleInternalModuleId::Type,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSysmoduleIsLoadedInternal(
        id: SceSysmoduleInternalModuleId::Type,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSysmoduleLoadModuleInternalWithArg(
        id: SceSysmoduleInternalModuleId::Type,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        option: *const SceSysmoduleOpt,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSysmoduleUnloadModuleInternalWithArg(
        id: SceSysmoduleInternalModuleId::Type,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        option: *const SceSysmoduleOpt,
    ) -> crate::ctypes::c_int;
}
