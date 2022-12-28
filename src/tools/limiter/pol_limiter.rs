/*
if threshold < 1.0 {
                threshold += 0.0002;
                if threshold > 1.0 {
                    threshold = 1.0;
                }
            }

            let mut non_clipped_out = vec![0.0; (available * (n_channels as i32)) as usize];
            for i in 0..out.len() {
                let mut val = out[i]*threshold;
                let downclip = val < -1.0;
                let upclip = val > 1.0;
                if downclip || upclip {
                    println!("Clipping");

                    let dif;

                    if upclip {
                        dif = val - 1.0;
                    } else {
                        dif = (val + 1.0)*-1.0;
                    }
                    threshold /= dif + 1.0;
                    let maxval = threshold/1.95;

                    val = if upclip { maxval } else { maxval*-1.0 };
                }

                non_clipped_out[i] = val;
            }*/