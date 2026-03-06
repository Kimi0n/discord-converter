# Discord Converter

Basically a ffmpeg GUI that lets you set a maximum filesize and converts a video with that in mind. Great for Discord's very generous filesize limits.

Only tested with Windows 11.

# Todos

- Create a settings tab with an encoder selector and hardware accelerator setting (h264, h265, av1, cpu or gpu)
- hdr to sdr tonemapping

- Config file that gets created when it doesnt exist yet
    - Configure defaults in settings
- Progressbar while ffmpeg is encoding

- Write tests and proper logging

# Things for me to remember

- Add the ffmpeg and ffprobe exes to ```src-tauri/bin```

## Start dev server

```npm run tauri dev```

## Build the thing

```npm run tauri build```