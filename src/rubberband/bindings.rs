#![allow(warnings)]

pub const RUBBERBAND_VERSION: &[u8; 6usize] = b"3.1.2\0";
pub const RUBBERBAND_API_MAJOR_VERSION: u32 = 2;
pub const RUBBERBAND_API_MINOR_VERSION: u32 = 7;
pub const RubberBandOption_RubberBandOptionProcessOffline: RubberBandOption = 0;
pub const RubberBandOption_RubberBandOptionProcessRealTime: RubberBandOption = 1;
pub const RubberBandOption_RubberBandOptionStretchElastic: RubberBandOption = 0;
pub const RubberBandOption_RubberBandOptionStretchPrecise: RubberBandOption = 16;
pub const RubberBandOption_RubberBandOptionTransientsCrisp: RubberBandOption = 0;
pub const RubberBandOption_RubberBandOptionTransientsMixed: RubberBandOption = 256;
pub const RubberBandOption_RubberBandOptionTransientsSmooth: RubberBandOption = 512;
pub const RubberBandOption_RubberBandOptionDetectorCompound: RubberBandOption = 0;
pub const RubberBandOption_RubberBandOptionDetectorPercussive: RubberBandOption = 1024;
pub const RubberBandOption_RubberBandOptionDetectorSoft: RubberBandOption = 2048;
pub const RubberBandOption_RubberBandOptionPhaseLaminar: RubberBandOption = 0;
pub const RubberBandOption_RubberBandOptionPhaseIndependent: RubberBandOption = 8192;
pub const RubberBandOption_RubberBandOptionThreadingAuto: RubberBandOption = 0;
pub const RubberBandOption_RubberBandOptionThreadingNever: RubberBandOption = 65536;
pub const RubberBandOption_RubberBandOptionThreadingAlways: RubberBandOption = 131072;
pub const RubberBandOption_RubberBandOptionWindowStandard: RubberBandOption = 0;
pub const RubberBandOption_RubberBandOptionWindowShort: RubberBandOption = 1048576;
pub const RubberBandOption_RubberBandOptionWindowLong: RubberBandOption = 2097152;
pub const RubberBandOption_RubberBandOptionSmoothingOff: RubberBandOption = 0;
pub const RubberBandOption_RubberBandOptionSmoothingOn: RubberBandOption = 8388608;
pub const RubberBandOption_RubberBandOptionFormantShifted: RubberBandOption = 0;
pub const RubberBandOption_RubberBandOptionFormantPreserved: RubberBandOption = 16777216;
pub const RubberBandOption_RubberBandOptionPitchHighSpeed: RubberBandOption = 0;
pub const RubberBandOption_RubberBandOptionPitchHighQuality: RubberBandOption = 33554432;
pub const RubberBandOption_RubberBandOptionPitchHighConsistency: RubberBandOption = 67108864;
pub const RubberBandOption_RubberBandOptionChannelsApart: RubberBandOption = 0;
pub const RubberBandOption_RubberBandOptionChannelsTogether: RubberBandOption = 268435456;
pub const RubberBandOption_RubberBandOptionEngineFaster: RubberBandOption = 0;
pub const RubberBandOption_RubberBandOptionEngineFiner: RubberBandOption = 536870912;
#[doc = " This is a C-linkage interface to the Rubber Band time stretcher."]
#[doc = ""]
#[doc = " This is a wrapper interface: the primary interface is in C++ and is"]
#[doc = " defined and documented in RubberBandStretcher.h.  The library"]
#[doc = " itself is implemented in C++, and requires C++ standard library"]
#[doc = " support even when using the C-linkage API."]
#[doc = ""]
#[doc = " Please see RubberBandStretcher.h for documentation."]
#[doc = ""]
#[doc = " If you are writing to the C++ API, do not include this header."]
pub type RubberBandOption = ::std::os::raw::c_int;
pub type RubberBandOptions = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RubberBandState_ {
    _unused: [u8; 0],
}
pub type RubberBandState = *mut RubberBandState_;
#[link(name="rubberband")]
extern "C" {
    pub fn rubberband_new(
        sampleRate: ::std::os::raw::c_uint,
        channels: ::std::os::raw::c_uint,
        options: RubberBandOptions,
        initialTimeRatio: f64,
        initialPitchScale: f64,
    ) -> RubberBandState;
}
extern "C" {
    pub fn rubberband_delete(arg1: RubberBandState);
}
extern "C" {
    pub fn rubberband_reset(arg1: RubberBandState);
}
extern "C" {
    pub fn rubberband_get_engine_version(arg1: RubberBandState) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rubberband_set_time_ratio(arg1: RubberBandState, ratio: f64);
}
extern "C" {
    pub fn rubberband_set_pitch_scale(arg1: RubberBandState, scale: f64);
}
extern "C" {
    pub fn rubberband_get_time_ratio(arg1: RubberBandState) -> f64;
}
extern "C" {
    pub fn rubberband_get_pitch_scale(arg1: RubberBandState) -> f64;
}
extern "C" {
    pub fn rubberband_set_formant_scale(arg1: RubberBandState, scale: f64);
}
extern "C" {
    pub fn rubberband_get_formant_scale(arg1: RubberBandState) -> f64;
}
extern "C" {
    pub fn rubberband_get_preferred_start_pad(arg1: RubberBandState) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rubberband_get_start_delay(arg1: RubberBandState) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rubberband_get_latency(arg1: RubberBandState) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rubberband_set_transients_option(arg1: RubberBandState, options: RubberBandOptions);
}
extern "C" {
    pub fn rubberband_set_detector_option(arg1: RubberBandState, options: RubberBandOptions);
}
extern "C" {
    pub fn rubberband_set_phase_option(arg1: RubberBandState, options: RubberBandOptions);
}
extern "C" {
    pub fn rubberband_set_formant_option(arg1: RubberBandState, options: RubberBandOptions);
}
extern "C" {
    pub fn rubberband_set_pitch_option(arg1: RubberBandState, options: RubberBandOptions);
}
extern "C" {
    pub fn rubberband_set_expected_input_duration(
        arg1: RubberBandState,
        samples: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn rubberband_get_samples_required(arg1: RubberBandState) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rubberband_set_max_process_size(arg1: RubberBandState, samples: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rubberband_set_key_frame_map(
        arg1: RubberBandState,
        keyframecount: ::std::os::raw::c_uint,
        from: *mut ::std::os::raw::c_uint,
        to: *mut ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn rubberband_study(
        arg1: RubberBandState,
        input: *const *const f32,
        samples: ::std::os::raw::c_uint,
        final_: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rubberband_process(
        arg1: RubberBandState,
        input: *const *const f32,
        samples: ::std::os::raw::c_uint,
        final_: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn rubberband_available(arg1: RubberBandState) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rubberband_retrieve(
        arg1: RubberBandState,
        output: *const *mut f32,
        samples: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rubberband_get_channel_count(arg1: RubberBandState) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn rubberband_calculate_stretch(arg1: RubberBandState);
}
extern "C" {
    pub fn rubberband_set_debug_level(arg1: RubberBandState, level: ::std::os::raw::c_int);
}
extern "C" {
    pub fn rubberband_set_default_debug_level(level: ::std::os::raw::c_int);
}
