use std::{fs::File, io::BufReader};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

pub struct OutputAudioStream {
    _output_stream: OutputStream,
    output_stream_handle: OutputStreamHandle,
}

pub struct AudioHelper {
    output_audio_stream: OutputAudioStream,
    audio_sink: Option<Sink>,
}

pub fn get_audio_helper() -> AudioHelper {
    let mut audio_helper = AudioHelper::new();
    audio_helper.init_sink();

    audio_helper
}

impl AudioHelper {
    fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            output_audio_stream: OutputAudioStream {
                _output_stream: _stream,
                output_stream_handle: stream_handle,
            },
            audio_sink: None,
        }
    }

    fn init_sink(&mut self) -> () {
        self.audio_sink = Option::from(Sink::try_new(&self.output_audio_stream.output_stream_handle).unwrap());
    }

    pub fn play_file(&mut self, path: &str) -> () {
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        self.audio_sink.as_ref().unwrap().append(source);
    }
}