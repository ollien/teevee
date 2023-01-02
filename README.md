# TeeVee

**NOTE:** This project is in what I'd call "weekend project" status. Behavior may change as I need soon, but I still wanted to share it. It is perhaps not the most perfect project code-quality and documentation wise, but I made it in an afternoon :)

While cutting the cord is in vogue these days, one thing we lose is the ability to just "turn on the TV" and watch whatever's on, which I've found can lead to decision paralysis. I decided to take this into my own hands, and built `teevee`.

`teevee` is fairly dependent on `ffmpeg` (which must be installed). At runtime, `teevee` will scan configured globs for files, and send them to an RTMP stream.

## Dependencies
- `ffmpeg` must be installed.
- You must have an RTMP server running somewhere. My current instance uses [Monaserver](https://github.com/MonaSolutions/MonaServer2).

## Usage
```
Usage: teevee [OPTIONS]

Options:
  -c <CONFIG_FILE>      [default: config.yml]
  -h, --help            Print help information
```

A configuration file is required (and defaults to `config.yml`). It follows the following format

```yml
ffmpeg_path: "/usr/bin/ffmpeg"
video_globs:
  - "/path/to/library/Futurama/**/*.mkv"
  - "/path/to/library/Seinfeld/**/*.mkv"
rtmp_uri: "rtmp://my.rtmp.server/live/key"
```

## Limitations
In order to not reach the limit on the number of characters in a command, files are encoded 100 at a time. There may be a disconnection in your RTMP stream every 100 files.

## Planned features
- [ ] "Schedule preview", just like real TV!
- [ ] `docker-compose` setup so the RMTP server doesn't have to be managed separately
- [ ] Perhaps some way to stop encoding when no one is watching?

