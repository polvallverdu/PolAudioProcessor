use sndfile::SndFileIO;

extern crate sndfile;
mod tools;
mod utils;

// This is for testing
/*fn main() {
    let mut song1 = sndfile::OpenOptions::ReadOnly(sndfile::ReadOptions::Auto).from_path("./hidden/puntog.wav").unwrap();
    let mut song1data: Vec<f32> = vec![0.0; 44100*10*2];
    song1.seek(std::io::SeekFrom::Current(44100*20*2));
    song1.read_to_slice(&mut song1data);
    println!("finished reading");

    let mut reverb = reverb::freeverb::Freeverb::new(44100);
    reverb.configure();
    let mut reverb_data = reverb.apply(song1data);
    limiter::simple_limiter::limit(&mut reverb_data);

    let mut file = sndfile::OpenOptions::WriteOnly(sndfile::WriteOptions::new(sndfile::MajorFormat::FLAC, sndfile::SubtypeFormat::PCM_16, sndfile::Endian::File, 44100, 2)).from_path("./hidden/output.wav").unwrap();
    file.write_from_slice(&reverb_data).unwrap();
}*/

fn main() {
    let mut song1 = sndfile::OpenOptions::ReadOnly(sndfile::ReadOptions::Auto).from_path("./hidden/puntog.wav").unwrap();
    let mut song1data: Vec<f32> = vec![0.0; 44100*10*2];
    song1.seek(std::io::SeekFrom::Current(44100*20*2)).unwrap();
    song1.read_to_slice(&mut song1data).unwrap();
    println!("finished reading");

    let mut rb = tools::RubberBand::new(44100, 2);
    let mut out_data: Vec<f32> = Vec::new();
    let mut finished = false;

    let mut pointer = 0;

    while !finished {
        let needed = rb.needed_samples();
        let mut to = pointer+(needed*2) as usize;
        if to > song1data.len() {
            to = song1data.len();
            finished = true;
        }

        let slice = &song1data[pointer..to as usize];
        let mut toconvert: Vec<Vec<f32>> = rb.normalize_input(slice.to_vec(), needed as usize);
        pointer = to;

        let mut converted = rb.normalize_output(&rb.process(&toconvert));
        out_data.append(&mut converted);
        if song1data.len() == 0 {
            finished = true;
        }
    }


    let mut file = sndfile::OpenOptions::WriteOnly(sndfile::WriteOptions::new(sndfile::MajorFormat::FLAC, sndfile::SubtypeFormat::PCM_16, sndfile::Endian::File, 44100, 2)).from_path("./hidden/output.wav").unwrap();
    file.write_from_slice(&out_data).unwrap();
}
