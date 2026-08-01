use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use mrlycore::Json;
use mrlymusic::render::{cycle, FADE};
use mrlymusic::wave::Wave;
use std::sync::{Arc, Mutex};

// MIXER

struct Voice {
    id: Option<String>,
    table: Vec<f32>,
    phase: f32,
    step: f32,
    gain: f32,
    elapsed: usize,
    release: Option<usize>,
    fade: usize,
}

impl Voice {
    fn level(&self) -> f32 {
        let fade = self.fade.max(1) as f32;
        let held = match self.release {
            Some(at) => self.elapsed.min(at),
            None => self.elapsed,
        };
        let up = (held as f32 / fade).min(1.0);
        let down = match self.release {
            Some(at) if self.elapsed > at => 1.0 - ((self.elapsed - at) as f32 / fade).min(1.0),
            _ => 1.0,
        };
        up * down
    }

    fn spent(&self) -> bool {
        matches!(self.release, Some(at) if self.elapsed >= at + self.fade)
    }
}

fn voice(
    id: Option<String>,
    hz: f32,
    wave: Wave,
    gain: f32,
    seconds: Option<f32>,
    rate: f32,
) -> Voice {
    let fade = (FADE * rate) as usize;
    let table = cycle(&wave, hz);
    let step = hz * table.len() as f32 / rate;
    let release = seconds.map(|s| ((s.max(2.0 * FADE) * rate) as usize).saturating_sub(fade));
    Voice {
        id,
        table,
        phase: 0.0,
        step,
        gain,
        elapsed: 0,
        release,
        fade,
    }
}

fn mix(voices: &mut Vec<Voice>, out: &mut [f32], channels: usize) {
    for frame in out.chunks_mut(channels.max(1)) {
        let mut sum = 0.0f32;
        for v in voices.iter_mut() {
            let len = v.table.len();
            let i = v.phase as usize;
            let a = v.table[i % len];
            let b = v.table[(i + 1) % len];
            let frac = v.phase - i as f32;
            sum += (a + (b - a) * frac) * v.gain * v.level();
            v.phase += v.step;
            if v.phase >= len as f32 {
                v.phase %= len as f32;
            }
            v.elapsed += 1;
        }
        let sample = sum.clamp(-1.0, 1.0);
        for slot in frame {
            *slot = sample;
        }
    }
    voices.retain(|v| !v.spent());
}

// DEVICE

pub struct Audio {
    voices: Arc<Mutex<Vec<Voice>>>,
    rate: f32,
    _stream: cpal::Stream,
}

impl Audio {
    pub fn new() -> Option<Audio> {
        let device = cpal::default_host().default_output_device()?;
        let config = device.default_output_config().ok()?;
        let rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;
        let voices: Arc<Mutex<Vec<Voice>>> = Arc::new(Mutex::new(Vec::new()));
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                open::<f32>(&device, &config.into(), voices.clone(), channels)
            }
            cpal::SampleFormat::I16 => {
                open::<i16>(&device, &config.into(), voices.clone(), channels)
            }
            cpal::SampleFormat::U16 => {
                open::<u16>(&device, &config.into(), voices.clone(), channels)
            }
            _ => None,
        }?;
        stream.play().ok()?;
        Some(Audio {
            voices,
            rate,
            _stream: stream,
        })
    }

    pub fn perform(&self, data: &Json) {
        let hz = data["freq"].as_i64().map(|f| f as f32 / 1000.0);
        let gain = data["gain"]
            .as_i64()
            .map(|g| g as f32 / 100.0)
            .unwrap_or(0.3);
        let wave = data["wave"]
            .as_str()
            .and_then(Wave::parse)
            .unwrap_or(Wave::Sine);
        let id = data["id"].as_str();
        match (data["op"].as_str(), hz, id) {
            (Some("note"), Some(hz), _) => {
                let seconds = data["ms"].as_i64().unwrap_or(150) as f32 / 1000.0;
                self.spawn(voice(None, hz, wave, gain, Some(seconds), self.rate));
            }
            (Some("start"), Some(hz), Some(id)) => {
                self.hush(id);
                self.spawn(voice(Some(id.to_string()), hz, wave, gain, None, self.rate));
            }
            (Some("stop"), _, Some(id)) => self.hush(id),
            _ => {}
        }
    }

    fn spawn(&self, voice: Voice) {
        if let Ok(mut voices) = self.voices.lock() {
            voices.push(voice);
        }
    }

    fn hush(&self, id: &str) {
        if let Ok(mut voices) = self.voices.lock() {
            for v in voices.iter_mut() {
                if v.id.as_deref() == Some(id) && v.release.is_none() {
                    v.release = Some(v.elapsed);
                }
            }
        }
    }
}

fn open<T: cpal::SizedSample + cpal::FromSample<f32>>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    voices: Arc<Mutex<Vec<Voice>>>,
    channels: usize,
) -> Option<cpal::Stream> {
    let mut scratch: Vec<f32> = Vec::new();
    device
        .build_output_stream(
            config,
            move |out: &mut [T], _: &cpal::OutputCallbackInfo| {
                scratch.clear();
                scratch.resize(out.len(), 0.0);
                if let Ok(mut voices) = voices.lock() {
                    mix(&mut voices, &mut scratch, channels);
                }
                for (slot, sample) in out.iter_mut().zip(&scratch) {
                    *slot = T::from_sample(*sample);
                }
            },
            |_| {},
            None,
        )
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attack_rises_from_silence() {
        let mut voices = vec![voice(None, 440.0, Wave::Sine, 1.0, None, 6400.0)];
        let mut out = vec![0.0f32; 100];
        mix(&mut voices, &mut out, 1);
        assert_eq!(out[0], 0.0);
        assert!(voices[0].level() >= 1.0 - 1e-6);
    }
    #[test]
    fn note_spends_itself() {
        let mut voices = vec![voice(None, 440.0, Wave::Sine, 1.0, Some(0.05), 6400.0)];
        let mut out = vec![0.0f32; 320];
        mix(&mut voices, &mut out, 1);
        assert!(voices.is_empty());
    }
    #[test]
    fn stop_fades_the_held_voice() {
        let mut voices = vec![voice(
            Some("piano:43".into()),
            440.0,
            Wave::Sine,
            1.0,
            None,
            6400.0,
        )];
        let mut out = vec![0.0f32; 200];
        mix(&mut voices, &mut out, 1);
        assert_eq!(voices.len(), 1);
        voices[0].release = Some(voices[0].elapsed);
        let mut out = vec![0.0f32; 100];
        mix(&mut voices, &mut out, 1);
        assert!(voices.is_empty());
    }
}
