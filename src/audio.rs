use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, SupportedStreamConfig, SupportedStreamConfigsError,
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

pub struct AudioRecorderBuilder {
    host: Arc<Host>,
    output_device_name: String,
    find_output_stream_config: Option<
        Box<
            dyn Fn(
                cpal::SupportedOutputConfigs,
            ) -> Result<SupportedStreamConfig, SupportedStreamConfigsError>,
        >,
    >,
}

impl AudioRecorderBuilder {
    pub fn new(host: Arc<Host>) -> Self {
        Self {
            host,
            output_device_name: String::default(),
            find_output_stream_config: Option::default(),
        }
    }

    pub fn with_output_device_name<S: Into<String>>(mut self, name: S) -> Self {
        self.output_device_name = name.into();
        self
    }

    pub fn with_output_stream_config(
        mut self,
        find: Box<
            dyn Fn(
                cpal::SupportedOutputConfigs,
            ) -> Result<SupportedStreamConfig, SupportedStreamConfigsError>,
        >,
    ) -> Self {
        self.find_output_stream_config = Some(find);
        self
    }

    pub fn build(self) -> Result<AudioRecorder, anyhow::Error> {
        let host = self.host.as_ref();

        let device: Device = host
            .output_devices()?
            .into_iter()
            .find(|device| device.name().ok().as_ref() == Some(&self.output_device_name))
            .or_else(|| host.default_output_device())
            .ok_or(anyhow!("ERROR: Device could not be found."))?; // TODO: Handle with custom error.

        let config: SupportedStreamConfig = match self.find_output_stream_config {
            Some(find) => find(device.supported_output_configs()?)?,
            None => device.default_output_config()?,
        };

        Ok(AudioRecorder { device, config })
    }
}

pub struct AudioRecorder {
    device: Device,
    config: SupportedStreamConfig,
}

pub struct AudioPlayerBuilder {
    host: Arc<Host>,
    input_device_name: String,
    find_input_stream_config: Option<
        Box<
            dyn Fn(
                cpal::SupportedInputConfigs,
            ) -> Result<SupportedStreamConfig, SupportedStreamConfigsError>,
        >,
    >,
}

impl AudioPlayerBuilder {
    pub fn new(host: Arc<Host>) -> Self {
        Self {
            host,
            input_device_name: String::default(),
            find_input_stream_config: Option::default(),
        }
    }

    pub fn with_input_device_name<S: Into<String>>(mut self, name: S) -> Self {
        self.input_device_name = name.into();
        self
    }

    pub fn with_output_stream_config(
        mut self,
        find: Box<
            dyn Fn(
                cpal::SupportedInputConfigs,
            ) -> Result<SupportedStreamConfig, SupportedStreamConfigsError>,
        >,
    ) -> Self {
        self.find_input_stream_config = Some(find);
        self
    }

    pub fn build(self) -> Result<AudioPlayer, anyhow::Error> {
        let host = self.host.as_ref();

        let device: Device = host
            .input_devices()?
            .into_iter()
            .find(|device| device.name().ok().as_ref() == Some(&self.input_device_name))
            .or_else(|| host.default_input_device())
            .ok_or(anyhow!("ERROR: Device could not be found."))?; // TODO: Handle with custom error.

        let config: SupportedStreamConfig = match self.find_input_stream_config {
            Some(find) => find(device.supported_input_configs()?)?,
            None => device.default_input_config()?,
        };

        Ok(AudioPlayer { device, config })
    }
}

pub struct AudioPlayer {
    device: Device,
    config: SupportedStreamConfig,
}
