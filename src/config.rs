use log::LevelFilter;
use serde::Deserialize;

/// Holds the configuration for `teevee`.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "defaults::ffmpeg_path")]
    ffmpeg_path: String,
    rtmp_uri: String,
    video_globs: Vec<String>,
    #[serde(default = "defaults::log_level")]
    log_level: LogLevel,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Eq))]
enum LogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}

impl Config {
    /// Get the path to the ffmpeg binary. If not specified, defaults to `/usr/bin/ffmpeg`.
    #[must_use]
    pub fn ffmpeg_path(&self) -> &str {
        &self.ffmpeg_path
    }

    /// Get the URI where the video RTMP should be streamed.
    #[must_use]
    pub fn rtmp_uri(&self) -> &str {
        &self.rtmp_uri
    }

    /// Get the paths to search for videos to play. These should be in glob form which correspond to files
    /// e.g. `/library/tv/Futurama/*.mp4` would be a valid entry for this configuration
    #[must_use]
    pub fn video_globs(&self) -> &[String] {
        &self.video_globs
    }

    #[must_use]
    pub fn log_level(&self) -> LevelFilter {
        match &self.log_level {
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warning => LevelFilter::Warn,
            LogLevel::Error => LevelFilter::Error,
        }
    }
}

mod defaults {
    use super::LogLevel;

    pub(super) fn log_level() -> LogLevel {
        LogLevel::Info
    }

    pub(super) fn ffmpeg_path() -> String {
        "/usr/bin/ffmpeg".to_string()
    }
}

#[cfg(test)]
mod tests {
    // Workaround for testcase
    #![allow(clippy::needless_pass_by_value)]

    use super::*;
    use std::mem;
    use test_case::test_case;

    #[test_case("error", LogLevel::Error)]
    #[test_case("warning", LogLevel::Warning)]
    #[test_case("info", LogLevel::Info)]
    #[test_case("debug", LogLevel::Debug)]
    fn test_parse_log_level(raw: &str, expected: LogLevel) {
        let parsed = serde_yaml::from_str::<LogLevel>(&format!(r#""{raw}""#))
            .unwrap_or_else(|err| panic!("failed to parse log level '{raw}': {err}"));

        assert!(
            mem::discriminant(&parsed) == mem::discriminant(&expected),
            "Expected {expected:?}, got {parsed:?}"
        );
    }
}
