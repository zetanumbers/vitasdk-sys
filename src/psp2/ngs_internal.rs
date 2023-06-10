/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

pub type SceNgsHRack = SceUInt32;
pub type SceNgsHPatch = SceUInt32;
pub type SceNgsHSynSystem = SceUInt32;
pub type SceNgsHVoice = SceUInt32;
pub type SulphaNgsModuleQueryType = SceUInt32;
pub type SceNgsModuleID = SceUInt32;
pub type SceNgsSulphaUpdateCallback = *mut crate::ctypes::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsCallbackInfo {
    _unused: [u8; 0],
}
pub type SceNgsCallbackFunc =
    ::core::option::Option<unsafe extern "C" fn(callback_info: *const SceNgsCallbackInfo)>;
pub type SceNgsRackReleaseCallbackFunc = SceNgsCallbackFunc;
pub type SceNgsModuleCallbackFunc = SceNgsCallbackFunc;
pub type SceNgsParamsErrorCallbackFunc = SceNgsCallbackFunc;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsVoicePreset {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsSystemInitParams {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsBufferInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsCallbackListInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SulphaNgsModuleQuery {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SulphaNgsRegistration {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsRackDescription {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsPatchSetupInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsParamsDescriptor {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsVoiceDefinition {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sceNgsModuleCheckParamsInRangeInternal(
        handle: SceNgsHVoice,
        module_id: SceNgsModuleID,
        descriptor: *const SceNgsParamsDescriptor,
        size: SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsModuleGetNumPresetsInternal(
        handle: SceNgsHSynSystem,
        module_id: SceNgsModuleID,
        num_presets: *mut SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsModuleGetPresetInternal(
        handle: SceNgsHSynSystem,
        module_id: SceNgsModuleID,
        preset_index: SceUInt32,
        info: *mut SceNgsBufferInfo,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsPatchCreateRoutingInternal(
        info: *const SceNgsPatchSetupInfo,
        handle: *mut SceNgsHPatch,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsPatchRemoveRoutingInternal(handle: SceNgsHPatch) -> SceInt32;
}
extern "C" {
    pub fn sceNgsRackGetRequiredMemorySizeInternal(
        handle: SceNgsHSynSystem,
        rack_description: *const SceNgsRackDescription,
        user_size: *mut SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsRackGetVoiceHandleInternal(
        rack_handle: SceNgsHRack,
        index: SceUInt32,
        voice_handle: *mut SceNgsHVoice,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsRackInitInternal(
        system_handle: SceNgsHSynSystem,
        rack_buffer: *mut SceNgsBufferInfo,
        rack_description: *const SceNgsRackDescription,
        rack_handle: *mut SceNgsHRack,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsRackReleaseInternal(
        handle: SceNgsHRack,
        callback: SceNgsRackReleaseCallbackFunc,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsRackSetParamErrorCallbackInternal(
        rack_handle: SceNgsHRack,
        callback: SceNgsParamsErrorCallbackFunc,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSulphaGetInfoInternal(
        obj_reg: *const SulphaNgsRegistration,
        info: *mut SceNgsBufferInfo,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSulphaGetModuleListInternal(
        module_ids: *mut SceUInt32,
        in_array_count: SceUInt32,
        count: *mut SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSulphaGetSynthUpdateCallbackInternal(
        handle: SceNgsHSynSystem,
        update_callback: *mut SceNgsSulphaUpdateCallback,
        info: *mut SceNgsBufferInfo,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSulphaQueryModuleInternal(
        type_: SulphaNgsModuleQueryType,
        debug: *mut SulphaNgsModuleQuery,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSulphaSetSynthUpdateCallbackInternal(
        handle: SceNgsHSynSystem,
        update_callback: SceNgsSulphaUpdateCallback,
        info: *mut SceNgsBufferInfo,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSystemGetCallbackListInternal(
        handle: SceNgsHSynSystem,
        array: *mut *mut SceNgsCallbackListInfo,
        array_size: *mut SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSystemGetRequiredMemorySizeInternal(
        params: *const SceNgsSystemInitParams,
        size: *mut SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSystemInitInternal(
        buffer_info: *mut SceNgsBufferInfo,
        compiled_sdk_version: SceUInt32,
        params: *const SceNgsSystemInitParams,
        handle: *mut SceNgsHSynSystem,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSystemLockInternal(handle: SceNgsHSynSystem) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSystemPullDataInternal(
        handle: SceNgsHSynSystem,
        dirty_flags_a: SceUInt32,
        dirty_flags_b: SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSystemPushDataInternal(handle: SceNgsHSynSystem) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSystemReleaseInternal(handle: SceNgsHSynSystem) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSystemSetFlagsInternal(
        handle: SceNgsHSynSystem,
        system_flags: SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSystemSetParamErrorCallbackInternal(
        handle: SceNgsHSynSystem,
        callback_id: SceNgsParamsErrorCallbackFunc,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSystemUnlockInternal(handle: SceNgsHSynSystem) -> SceInt32;
}
extern "C" {
    pub fn sceNgsSystemUpdateInternal(handle: SceNgsHSynSystem) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceBypassModuleInternal(
        handle: SceNgsHVoice,
        module: SceUInt32,
        flag: SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceClearDirtyFlagInternal(
        handle: SceNgsHVoice,
        param_bit_flag: SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceDefinitionGetPresetInternal(
        definition: *const SceNgsVoiceDefinition,
        index: SceUInt32,
        presets: *mut *const SceNgsVoicePreset,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceGetModuleBypassInternal(
        handle: SceNgsHVoice,
        module: SceUInt32,
        flag: *mut SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceGetOutputPatchInternal(
        handle: SceNgsHVoice,
        nOutputIndex: SceInt32,
        nSubIndex: SceInt32,
        pPatchHandle: *mut SceNgsHPatch,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceGetParamsOutOfRangeBufferedInternal(
        handle: SceNgsHVoice,
        module: SceUInt32,
        message_buffer: *mut crate::ctypes::c_char,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceInitInternal(
        handle: SceNgsHVoice,
        preset: *const SceNgsVoicePreset,
        flags: SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceKeyOffInternal(handle: SceNgsHVoice) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceKillInternal(handle: SceNgsHVoice) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoicePauseInternal(handle: SceNgsHVoice) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoicePlayInternal(handle: SceNgsHVoice) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceResumeInternal(handle: SceNgsHVoice) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceSetAllBypassesInternal(handle: SceNgsHVoice, bitflag: SceUInt32) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceSetFinishedCallbackInternal(
        handle: SceNgsHVoice,
        callback: SceNgsCallbackFunc,
        userdata: *mut crate::ctypes::c_void,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceSetModuleCallbackInternal(
        handle: SceNgsHVoice,
        module: SceUInt32,
        callback: SceNgsModuleCallbackFunc,
        callback_userdata: *mut crate::ctypes::c_void,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceSetPresetInternal(
        handle: SceNgsHVoice,
        preset: *const SceNgsVoicePreset,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceNgsVoiceDefGetAtrac9VoiceInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetCompressorBussInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetCompressorSideChainBussInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetDelayBussInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetDistortionBussInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetEnvelopeBussInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetEqBussInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetMasterBussInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetMixerBussInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetPauserBussInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetPitchshiftBussInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetReverbBussInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetSasEmuVoiceInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetScreamVoiceAT9Internal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetScreamVoiceInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetSimpleAtrac9VoiceInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetSimpleVoiceInternal() -> *const SceNgsVoiceDefinition;
}
extern "C" {
    pub fn sceNgsVoiceDefGetTemplate1Internal() -> *const SceNgsVoiceDefinition;
}