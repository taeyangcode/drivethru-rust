use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, SupportedStreamConfig,
};
use strum::AsRefStr;

#[derive(AsRefStr)]
pub enum SpeechModel {
    TTSModel(TTSModel),
    STTModel(STTModel),
}

#[derive(AsRefStr)]
pub enum TTSModel {
    #[strum(serialize = "tts-1")]
    TTS1,
    #[strum(serialize = "tts-1-hd")]
    TTS1HD,
}

#[derive(AsRefStr)]
pub enum STTModel {
    #[strum(serialize = "whisper-1")]
    Whisper1,
}

#[derive(AsRefStr)]
pub enum TTSVoice {
    #[strum(serialize = "alloy")]
    Alloy,
    #[strum(serialize = "echo")]
    Echo,
    #[strum(serialize = "fable")]
    Fable,
    #[strum(serialize = "onyx")]
    Onyx,
    #[strum(serialize = "nova")]
    Nova,
    #[strum(serialize = "shimmer")]
    Shimmer,
}

const TTS_MODEL: TTSModel = TTSModel::TTS1HD;
const TTS_VOICE: TTSVoice = TTSVoice::Onyx;
const TTS_SAMPLE_RATE: usize = 24000;

const STT_MODEL: STTModel = STTModel::Whisper1;
const STT_SAMPLE_RATE: usize = 44100;
const STT_SILENCE_THRESHOLD: usize = 1500;
const STT_WAIT_SEC: std::time::Duration = std::time::Duration::from_secs(5);

pub struct AudioDevice {
    host: Host,
    device: Device,
    input_config: SupportedStreamConfig,
    buffer: Vec<f32>,
}

impl AudioDevice {
    pub fn new() -> Self {
        let host: Host = cpal::default_host();
        let device: Device = host.default_input_device().unwrap();
        let supported_config = device
            .supported_input_configs()
            .unwrap()
            .next()
            .unwrap()
            .with_max_sample_rate();

        println!("{}", device.name().unwrap());

        Self {
            host,
            device,
            input_config: supported_config,
            buffer: vec![],
        }
    }

    pub fn record_audio(&self) {
        let buffer: Arc<Mutex<Option<Vec<f32>>>> = Arc::new(Mutex::new(Some(vec![])));
        let stream_buffer = buffer.clone();

        let stream = self
            .device
            .build_input_stream(
                &self.input_config.config(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if let Ok(mut guard) = stream_buffer.try_lock() {
                        if let Some(buffer) = guard.as_mut() {
                            for &data in data.iter() {
                                buffer.push(data);
                            }
                        }
                    }
                },
                move |error| {
                    dbg!(error);
                },
                None,
            )
            .unwrap();

        stream.play().unwrap();

        std::thread::sleep(std::time::Duration::from_secs(5));
        drop(stream);
    }
}
