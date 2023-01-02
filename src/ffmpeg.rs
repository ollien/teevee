use std::{
    io,
    process::{Command, ExitStatus},
};

use itertools::Itertools;

pub fn stream_files(ffmpeg_path: &str, rtmp_uri: &str, files: &[&str]) -> io::Result<ExitStatus> {
    let mut cmd = Command::new(ffmpeg_path);
    let input_args = build_input_file_args(files);
    let complex_filter_args = build_complex_filter_args(files);
    let output_args = build_output_args(rtmp_uri);

    input_args
        .into_iter()
        .chain(complex_filter_args.iter().map(String::as_ref))
        .chain(output_args.iter().map(String::as_ref))
        .for_each(|arg| {
            cmd.arg(arg);
        });

    // TODO: See if we can get this piped through the logger
    cmd.status()
}

fn build_input_file_args<'a>(files: &[&'a str]) -> Vec<&'a str> {
    let i_flag_iter = files.iter().copied().flat_map(|file| ["-i", file]);

    let mut res = vec!["-re"];
    res.extend(i_flag_iter);

    res
}

fn build_complex_filter_args(files: &[&str]) -> Vec<String> {
    vec![
        "-filter_complex".to_string(),
        build_complex_filter(files),
        "-map".to_string(),
        "[v]".to_string(),
        "-map".to_string(),
        "[a]".to_string(),
    ]
}

fn build_complex_filter(files: &[&str]) -> String {
    let scale = build_scale_filter(files);
    let concat = build_concat_filter(files);
    format!("{scale}; {concat}")
}

fn build_scale_filter(files: &[&str]) -> String {
    files
        .iter()
        .copied()
        .enumerate()
        .map(|(idx, _file)| format!("[{idx}:v]scale=1920:1080:force_original_aspect_ratio=decrease,setsar=1:1,pad=1920:1080:-1:-1:color=black[v{idx}]"))
        .join("; ")
}

fn build_concat_filter(files: &[&str]) -> String {
    let inputs = files
        .iter()
        .copied()
        .enumerate()
        .map(|(idx, _file)| format!("[v{idx}] [{idx}:a]"))
        .join(" ");

    let num_files = files.len();
    format!("{inputs} concat=n={num_files}:v=1:a=1 [v] [a]")
}

fn build_output_args(rtmp_url: &str) -> Vec<String> {
    vec![
        "-vcodec".to_string(),
        "libx264".to_string(),
        "-f".to_string(),
        "flv".to_string(),
        rtmp_url.to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_input_file_args() {
        let files = ["a.mp4", "b.mkv", "c.flv"];
        let args = build_input_file_args(&files);
        assert_eq!(&args, &["-re", "-i", "a.mp4", "-i", "b.mkv", "-i", "c.flv"]);
    }

    #[test]
    fn test_build_complex_filter_args() {
        let files = ["a.mp4", "b.mkv", "c.flv"];
        let args = build_complex_filter_args(&files);
        assert_eq!(&args, &[
            "-filter_complex".to_string(),
            "[0:v]scale=1920:1080:force_original_aspect_ratio=decrease:flags=lanczos,setsar=1:1,pad=1920:1080:-1:-1:color=black[v0]; \
            [1:v]scale=1920:1080:force_original_aspect_ratio=decrease:flags=lanczos,setsar=1:1,pad=1920:1080:-1:-1:color=black[v1]; \
            [2:v]scale=1920:1080:force_original_aspect_ratio=decrease:flags=lanczos,setsar=1:1,pad=1920:1080:-1:-1:color=black[v2]; \
            [v0] [0:a] [v1] [1:a] [v2] [2:a] concat=n=3:v=1:a=1 [v] [a]".to_string(),
            "-map".to_string(),
            "[v]".to_string(),
            "-map".to_string(),
            "[a]".to_string()
            ]
        );
    }

    #[test]
    fn test_build_output_args() {
        let rtmp_url = "rtmp://127.0.0.1/live/stream";
        let args = build_output_args(rtmp_url);
        assert_eq!(
            &args,
            &[
                "-vcodec".to_string(),
                "libx264".to_string(),
                "-f".to_string(),
                "flv".to_string(),
                "rtmp://127.0.0.1/live/stream".to_string()
            ]
        );
    }
}
