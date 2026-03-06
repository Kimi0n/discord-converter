# Discord Converter

Basically a ffmpeg GUI that lets you set a maximum filesize and converts a video with that in mind. Great for Discord's very generous filesize limits.

Only tested with Windows 11.

# Todos

- Remember the settings that were set last time by saving them in a config file
- Progressbar while ffmpeg is encoding

- Create a settings tab with an encoder selector and hardware accelerator setting (h264, h265, av1, cpu or gpu)
- Create a framerate selector (keep, 30, 60)
- Create a resolution selector (360p - 1440p)

- Write tests and proper logging

# Things for me to remember

- Add the ffmpeg and ffprobe exes to ```src-tauri/bin```

## Start dev server

```npm run tauri dev```

## Build the thing

```npm run tauri build```