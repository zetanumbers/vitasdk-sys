/* automatically generated by rust-bindgen 0.68.1 */

pub const SCE_SYSTEM_PARAM_USERNAME_MAXSIZE: u32 = 17;
pub mod SceSystemParamId {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SYSTEM_PARAM_ID_LANG: Type = 1;
    pub const SCE_SYSTEM_PARAM_ID_ENTER_BUTTON: Type = 2;
    pub const SCE_SYSTEM_PARAM_ID_USERNAME: Type = 3;
    pub const SCE_SYSTEM_PARAM_ID_DATE_FORMAT: Type = 4;
    pub const SCE_SYSTEM_PARAM_ID_TIME_FORMAT: Type = 5;
    pub const SCE_SYSTEM_PARAM_ID_TIME_ZONE: Type = 6;
    pub const SCE_SYSTEM_PARAM_ID_DAYLIGHT_SAVINGS: Type = 7;
    pub const SCE_SYSTEM_PARAM_ID_MAX_VALUE: Type = 4294967295;
}
pub mod SceSystemParamLang {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SYSTEM_PARAM_LANG_JAPANESE: Type = 0;
    pub const SCE_SYSTEM_PARAM_LANG_ENGLISH_US: Type = 1;
    pub const SCE_SYSTEM_PARAM_LANG_FRENCH: Type = 2;
    pub const SCE_SYSTEM_PARAM_LANG_SPANISH: Type = 3;
    pub const SCE_SYSTEM_PARAM_LANG_GERMAN: Type = 4;
    pub const SCE_SYSTEM_PARAM_LANG_ITALIAN: Type = 5;
    pub const SCE_SYSTEM_PARAM_LANG_DUTCH: Type = 6;
    pub const SCE_SYSTEM_PARAM_LANG_PORTUGUESE_PT: Type = 7;
    pub const SCE_SYSTEM_PARAM_LANG_RUSSIAN: Type = 8;
    pub const SCE_SYSTEM_PARAM_LANG_KOREAN: Type = 9;
    pub const SCE_SYSTEM_PARAM_LANG_CHINESE_T: Type = 10;
    pub const SCE_SYSTEM_PARAM_LANG_CHINESE_S: Type = 11;
    pub const SCE_SYSTEM_PARAM_LANG_FINNISH: Type = 12;
    pub const SCE_SYSTEM_PARAM_LANG_SWEDISH: Type = 13;
    pub const SCE_SYSTEM_PARAM_LANG_DANISH: Type = 14;
    pub const SCE_SYSTEM_PARAM_LANG_NORWEGIAN: Type = 15;
    pub const SCE_SYSTEM_PARAM_LANG_POLISH: Type = 16;
    pub const SCE_SYSTEM_PARAM_LANG_PORTUGUESE_BR: Type = 17;
    pub const SCE_SYSTEM_PARAM_LANG_ENGLISH_GB: Type = 18;
    pub const SCE_SYSTEM_PARAM_LANG_TURKISH: Type = 19;
    pub const SCE_SYSTEM_PARAM_LANG_MAX_VALUE: Type = 4294967295;
}
pub mod SceSystemParamEnterButtonAssign {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SYSTEM_PARAM_ENTER_BUTTON_CIRCLE: Type = 0;
    pub const SCE_SYSTEM_PARAM_ENTER_BUTTON_CROSS: Type = 1;
    pub const SCE_SYSTEM_PARAM_ENTER_BUTTON_MAX_VALUE: Type = 4294967295;
}
pub mod SceSystemParamDateFormat {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SYSTEM_PARAM_DATE_FORMAT_YYYYMMDD: Type = 0;
    pub const SCE_SYSTEM_PARAM_DATE_FORMAT_DDMMYYYY: Type = 1;
    pub const SCE_SYSTEM_PARAM_DATE_FORMAT_MMDDYYYY: Type = 2;
}
pub mod SceSystemParamTimeFormat {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SYSTEM_PARAM_TIME_FORMAT_12HR: Type = 0;
    pub const SCE_SYSTEM_PARAM_TIME_FORMAT_24HR: Type = 1;
}
