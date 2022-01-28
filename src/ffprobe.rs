//! ffprobe logic
use anyhow::Context;
use std::{path::Path, time::Duration};

pub struct Ffprobe {
    /// Duration of video
    pub duration: Duration,
    /// The video has audio stream.
    pub has_audio: bool,
}

pub fn probe(input: &Path) -> anyhow::Result<Ffprobe> {
    let probe = ffprobe::ffprobe(&input)?;

    let duration_s = probe
        .format
        .duration
        .as_deref()
        .context("ffprobe missing video duration")?
        .parse::<f32>()
        .context("invalid ffprobe video duration")?;

    Ok(Ffprobe {
        duration: Duration::from_secs_f32(duration_s),
        has_audio: probe
            .streams
            .iter()
            .any(|s| s.codec_type.as_deref() == Some("audio")),
    })
}
