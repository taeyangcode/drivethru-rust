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

struct AudioRecorder {}
