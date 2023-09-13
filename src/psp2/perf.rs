/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

pub const SCE_PERF_ARM_PMON_THREAD_ID_SELF: u32 = 0;
pub mod _ScePerfArmPmonEventCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_PERF_ARM_PMON_SOFT_INCREMENT: Type = 0;
    pub const SCE_PERF_ARM_PMON_ICACHE_MISS: Type = 1;
    pub const SCE_PERF_ARM_PMON_ITLB_MISS: Type = 2;
    pub const SCE_PERF_ARM_PMON_DCACHE_MISS: Type = 3;
    pub const SCE_PERF_ARM_PMON_DCACHE_ACCESS: Type = 4;
    pub const SCE_PERF_ARM_PMON_DTLB_MISS: Type = 5;
    pub const SCE_PERF_ARM_PMON_DATA_READ: Type = 6;
    pub const SCE_PERF_ARM_PMON_DATA_WRITE: Type = 7;
    pub const SCE_PERF_ARM_PMON_EXCEPTION_TAKEN: Type = 9;
    pub const SCE_PERF_ARM_PMON_EXCEPTION_RETURN: Type = 10;
    pub const SCE_PERF_ARM_PMON_WRITE_CONTEXTID: Type = 11;
    pub const SCE_PERF_ARM_PMON_SOFT_CHANGEPC: Type = 12;
    pub const SCE_PERF_ARM_PMON_IMMEDIATE_BRANCH: Type = 13;
    pub const SCE_PERF_ARM_PMON_UNALIGNED: Type = 15;
    pub const SCE_PERF_ARM_PMON_BRANCH_MISPREDICT: Type = 16;
    pub const SCE_PERF_ARM_PMON_CYCLE_COUNT: Type = 17;
    pub const SCE_PERF_ARM_PMON_PREDICT_BRANCH: Type = 18;
    pub const SCE_PERF_ARM_PMON_COHERENT_LF_MISS: Type = 80;
    pub const SCE_PERF_ARM_PMON_COHERENT_LF_HIT: Type = 81;
    pub const SCE_PERF_ARM_PMON_ICACHE_STALL: Type = 96;
    pub const SCE_PERF_ARM_PMON_DCACHE_STALL: Type = 97;
    pub const SCE_PERF_ARM_PMON_MAINTLB_STALL: Type = 98;
    pub const SCE_PERF_ARM_PMON_STREX_PASSED: Type = 99;
    pub const SCE_PERF_ARM_PMON_STREX_FAILED: Type = 100;
    pub const SCE_PERF_ARM_PMON_DATA_EVICTION: Type = 101;
    pub const SCE_PERF_ARM_PMON_ISSUE_NO_DISPATCH: Type = 102;
    pub const SCE_PERF_ARM_PMON_ISSUE_EMPTY: Type = 103;
    pub const SCE_PERF_ARM_PMON_INST_RENAME: Type = 104;
    pub const SCE_PERF_ARM_PMON_PREDICT_FUNC_RET: Type = 110;
    pub const SCE_PERF_ARM_PMON_MAIN_PIPE: Type = 112;
    pub const SCE_PERF_ARM_PMON_SECOND_PIPE: Type = 113;
    pub const SCE_PERF_ARM_PMON_LS_PIPE: Type = 114;
    pub const SCE_PERF_ARM_PMON_FPU_RENAME: Type = 115;
    pub const SCE_PERF_ARM_PMON_NEON_RENAME: Type = 116;
    pub const SCE_PERF_ARM_PMON_PLD_STALL: Type = 128;
    pub const SCE_PERF_ARM_PMON_WRITE_STALL: Type = 129;
    pub const SCE_PERF_ARM_PMON_INST_MAINTLB_STALL: Type = 130;
    pub const SCE_PERF_ARM_PMON_DATA_MAINTLB_STALL: Type = 131;
    pub const SCE_PERF_ARM_PMON_INST_UTLB_STALL: Type = 132;
    pub const SCE_PERF_ARM_PMON_DATA_UTLB_STALL: Type = 133;
    pub const SCE_PERF_ARM_PMON_DMB_STALL: Type = 134;
    pub const SCE_PERF_ARM_PMON_INTEGER_CLOCK: Type = 138;
    pub const SCE_PERF_ARM_PMON_DATAENGINE_CLOCK: Type = 139;
    pub const SCE_PERF_ARM_PMON_ISB: Type = 144;
    pub const SCE_PERF_ARM_PMON_DSB: Type = 145;
    pub const SCE_PERF_ARM_PMON_DMB: Type = 146;
    pub const SCE_PERF_ARM_PMON_EXT_INTERRUPT: Type = 147;
    pub const SCE_PERF_ARM_PMON_PLE_LINE_REQ_COMPLETED: Type = 160;
    pub const SCE_PERF_ARM_PMON_PLE_CHANNEL_SKIPPED: Type = 161;
    pub const SCE_PERF_ARM_PMON_PLE_FIFO_FLUSH: Type = 162;
    pub const SCE_PERF_ARM_PMON_PLE_REQ_COMPLETED: Type = 163;
    pub const SCE_PERF_ARM_PMON_PLE_FIFO_OVERFLOW: Type = 164;
    pub const SCE_PERF_ARM_PMON_PLE_REQ_PROGRAMMED: Type = 165;
}
pub use self::_ScePerfArmPmonEventCode::Type as ScePerfArmPmonEventCode;
extern "C" {
    pub fn scePerfArmPmonReset(thid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePerfArmPmonSelectEvent(
        thid: SceUID,
        counter: SceUInt32,
        event_code: SceUInt8,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePerfArmPmonStart(thid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePerfArmPmonStop(thid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePerfArmPmonGetCounterValue(
        thid: SceUID,
        counter: SceUInt32,
        value: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePerfArmPmonSetCounterValue(
        thid: SceUID,
        counter: SceUInt32,
        value: SceUInt32,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePerfArmPmonSoftwareIncrement(mask: SceUInt32) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePerfGetTimebaseValue() -> SceUInt64;
}
extern "C" {
    pub fn scePerfGetTimebaseFrequency() -> SceUInt32;
}
