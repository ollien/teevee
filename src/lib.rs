#![warn(clippy::all, clippy::pedantic)]

#[macro_use]
extern crate log;

pub use config::Config;

use glob::{GlobError, PatternError};
use itertools::Itertools;
use rand::seq::SliceRandom;
use std::path::PathBuf;
use thiserror::Error;

mod config;
mod ffmpeg;

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("Failed to parse globs: {0}")]
    GlobError(PatternError),
    #[error("Failed to load file globs: {0}")]
    FileLoadFailed(GlobError),
}

/// Stream all of the videos in the `Config` to its RTMP path.
///
/// # Errors
/// Returns `SetupError` if the stream setup fails. After setup, all errors will be logged.
pub fn stream_videos(config: &Config) -> Result<(), SetupError> {
    let paths = resolve_files(config.video_globs())?;
    let mut path_strs = paths
        .iter()
        .filter_map(|path| {
            let str_path = path.to_str();
            if str_path.is_none() {
                warn!("{path:?} is not readable as utf8, skipping");
            }
            str_path
        })
        .collect::<Vec<_>>();

    let mut rng = rand::thread_rng();
    loop {
        path_strs.shuffle(&mut rng);
        for path_chunk in &path_strs.iter().copied().chunks(100) {
            let path_chunk_copy = path_chunk.collect::<Vec<_>>();
            let stream_res =
                ffmpeg::stream_files(config.ffmpeg_path(), config.rtmp_uri(), &path_chunk_copy);
            if let Err(err) = stream_res {
                error!("Failed to stream files, retrying - status code: {err:?}");
            }
        }
    }
}

fn resolve_files<S: AsRef<str>>(globs: &[S]) -> Result<Vec<PathBuf>, SetupError> {
    let path_iterators = globs
        .iter()
        .map(S::as_ref)
        .map(glob::glob)
        .collect::<Result<Vec<_>, _>>()
        .map_err(SetupError::GlobError)?;

    path_iterators
        .into_iter()
        .flatten()
        .collect::<Result<Vec<_>, _>>()
        .map_err(SetupError::FileLoadFailed)
}
