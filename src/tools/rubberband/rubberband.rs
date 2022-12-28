use super::bindings::*;
use std::{thread, time};
use crate::tools::Limiter;

fn convert_to_vec_const(vec: &Vec<Vec<f32>>) -> Vec<*const f32> {
    let mut vec_const: Vec<*const f32> = Vec::with_capacity(vec.len());
    for inner in vec {
        vec_const.push(inner.as_ptr());
    }
    vec_const
}
pub struct RubberBand {
    rb: *mut RubberBandState_,

    samplerate: u32,
    channels: u32,
}

impl RubberBand {
    pub fn new(samplerate: u32, channels: u32) -> RubberBand {
        let rb;
        unsafe { rb = rubberband_new(samplerate, channels, 536870913, f64::from(1), f64::from(1)) }
        RubberBand {
            rb,
            samplerate,
            channels,
        }
    }

    pub fn needed_samples(&self) -> u32 {
        let needed_samples;
        unsafe { needed_samples = rubberband_get_samples_required(self.rb) }
        needed_samples
    }

    pub fn process(&self, toconvert: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        let mut output: Vec<Vec<f32>> = Vec::with_capacity(self.channels as usize);
        let mut available: i32;
        unsafe {
            rubberband_process(
                self.rb,
                convert_to_vec_const(toconvert).as_ptr(),
                toconvert[0].len() as u32,
                0,
            );

            available = rubberband_available(self.rb);
            while available == 0 {
                thread::sleep(time::Duration::from_millis(10));
                available = rubberband_available(self.rb);
            }

            let mut retrieved: Vec<*mut f32> = Vec::with_capacity(self.channels as usize);
            for _ in 0..self.channels {
                let channel_vec: *mut f32 =
                    Box::leak(vec![0.0; available as usize].into_boxed_slice()).as_mut_ptr();
                retrieved.push(channel_vec);
            }
            rubberband_retrieve(self.rb, retrieved.as_ptr(), available as u32);
            for i in 0..(self.channels as usize) {
                let output_slice = std::slice::from_raw_parts_mut(retrieved[i], available as usize);
                output.push(output_slice.to_vec());
            }
        }

        return output;
    }

    pub fn normalize_input(&self, input: Vec<f32>, samples: usize) -> Vec<Vec<f32>> {
        let mut out = vec![vec![0.0; samples]; self.channels as usize];

        for i in 0..input.len() {
            out[i % (self.channels as usize)][i / (self.channels as usize)] = input[i];
        }

        return out;
    }

    pub fn normalize_output(&self, output: &Vec<Vec<f32>>) -> Vec<f32> {
        let mut out = vec![0.0; output[0].len() * (self.channels as usize)];

        for i in 0..output.len() {
            for j in 0..output[i].len() {
                out[j * (self.channels as usize) + i] = output[i][j];
            }
        }
        Limiter::limit(&mut out);
        return out;
    }

    pub fn stop(&self) {
        unsafe { rubberband_delete(self.rb) }
    }

    pub fn set_time_ratio(&self, ratio: f64) {
        unsafe { rubberband_set_time_ratio(self.rb, ratio) }
    }

    pub fn set_pitch_scale(&self, scale: f64) {
        unsafe { rubberband_set_pitch_scale(self.rb, scale) }
    }

    pub fn get_time_ratio(&self) -> f64 {
        let ratio;
        unsafe { ratio = rubberband_get_time_ratio(self.rb) }
        ratio
    }

    pub fn get_pitch_scale(&self) -> f64 {
        let scale;
        unsafe { scale = rubberband_get_pitch_scale(self.rb) }
        scale
    }
}
