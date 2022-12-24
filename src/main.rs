use sndfile::SndFileIO;

extern crate sndfile;
mod rubberband;
mod mixer;
mod limiter;
mod utils;
mod reverb;

// This is for testing
fn main() {
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
}
