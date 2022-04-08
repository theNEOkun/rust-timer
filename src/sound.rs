use super::errors::*;
use rodio::{decoder::DecoderError, Decoder, OutputStream, Sink};
use std::{ffi::OsStr, fs::File, io::BufReader, path::PathBuf};

pub fn play_sound(sound_pos: PathBuf) -> PlaybackResult {
    if let Ok(sound) = get_sound(&sound_pos) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        match Sink::try_new(&stream_handle) {
            Ok(sink) => {
                sink.append(sound);
                sink.sleep_until_end();
                Ok(true)
            }
            Err(_) => Err(PlaybackError),
        }
    } else {
        Err(PlaybackError)
    }
}

fn get_sound(sound: &PathBuf) -> Result<Decoder<BufReader<File>>, DecoderError> {
    match File::open(sound.clone()) {
        Ok(file) => get_buf_reader(file, sound.extension()),
        Err(_) => panic!(""),
    }
}

fn get_buf_reader(
    file: File,
    sound: Option<&OsStr>,
) -> Result<Decoder<BufReader<File>>, DecoderError> {
    match sound {
        Some(extension) => match extension.to_str().unwrap() {
            "wav" => Decoder::new_wav(BufReader::new(file)),
            "mp3" => Decoder::new_mp3(BufReader::new(file)),
            _ => Decoder::new(BufReader::new(file)),
        },
        None => Decoder::new(BufReader::new(file)),
    }
}
