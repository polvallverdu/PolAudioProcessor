use freeverb::Freeverb as fv;
const CHANNELS: usize = 2;

pub struct Freeverb {
    freeverb: fv,
}

impl Freeverb {
    pub fn new(samplerate: usize) -> Freeverb {
        Freeverb {
            freeverb: fv::new(samplerate as usize),
        }
    }

    pub fn configure(&mut self) {
        self.freeverb.set_dry(-4.39);
        self.freeverb.set_wet(-3.91);
        self.freeverb.set_room_size(0.910);
        self.freeverb.set_dampening(0.43);
        self.freeverb.set_freeze(false);
        self.freeverb.set_width(0.97);
    }

    pub fn apply(&mut self, data: Vec<f32>) -> Vec<f32> {
        let mut out_reverb = vec![0.0; data.len()];
    
        for i in 0..(data.len() / CHANNELS) {
            let out = self.freeverb.tick((data[i * CHANNELS] as f64, data[i * CHANNELS + 1] as f64));
            out_reverb[i * CHANNELS] = out.0 as f32;
            out_reverb[i * CHANNELS + 1] = out.1 as f32;
        }
    
        return out_reverb;
    }
}
