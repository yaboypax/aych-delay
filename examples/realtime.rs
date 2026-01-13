extern crate aych_delay;

use aych_delay::{Delay, Settings};
use rodio::{buffer::SamplesBuffer, Decoder};
use std::fs::File;
use std::path::Path;

fn main() {
    // Get the audio file name from the command line arguments,
    // or exit early if no file name was provided
    let file_name = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: cargo run --example basic <audio_file>");
        std::process::exit(1);
    });

    // Exit early if the audio file doesn't exist
    if !Path::new(&file_name).exists() {
        println!("File not found: {}", file_name);
        std::process::exit(1);
    }

    let banner = include_str!("../banner.txt");
    println!("{}", banner);

    let mut delay = Delay::new(Settings {
        delay_time: 1666.66,
        feedback: 0.75,
        width: 0.5,
        lowpass_filter: 22000.0,
        highpass_filter: 300.0,
        dry_wet_mix: 0.5,
        output_level: 0.75,
        ..Settings::default()
    });

    let file = File::open(file_name).unwrap();
    let mut source = Decoder::new_looped(file).unwrap();

    // Get a output stream handle to the default physical sound device
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");

    // Create a sink to play samples
    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    // Create process block buffer
    let mut buffer = vec![0.0f32; 2048];

    loop {
        if sink.len() >= 2 {
            continue;
        }

        // fill one interleaved block
        for sample in buffer.iter_mut() {
            *sample = source.next().unwrap_or(0.0);
        }

        // process in realtime without vector allocation
        delay.process_realtime(&mut buffer);

        // play the output buffer
        sink.append(SamplesBuffer::new(2, 44_100, buffer.clone()));
    }
}
