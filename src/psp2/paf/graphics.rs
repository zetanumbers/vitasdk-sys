/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

extern "C" {
    pub fn scePafGraphicsUpdateCurrentWave(
        index: SceUInt32,
        update_interval: SceFloat32,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub static mut scePafGraphicsCurrentWave: SceUInt32;
}