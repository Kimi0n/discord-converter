# Discord Converter

Basically a ffmpeg GUI that lets you set a maximum filesize and converts a video with that in mind. Great for Discord's very generous filesize limits.

Only tested with Windows 11.

# Todos

- Implement optional h265 conversion
- Create a settings tab with a hardware accelerator setting (cpu or gpu)
- HDR to SDR tonemapping
- Figure out why commandline flashes for a split second
- Config file that gets created when it doesnt exist yet
    - Configure defaults in settings
- Progressbar while ffmpeg is encoding
- pass ffmpeg errors to the UI (like not found)
- Write tests and proper logging
- Create a custom icon for the program

# Things for me to remember

- Add the ffmpeg and ffprobe exes to ```src-tauri/bin```

## Start dev server

```npm run tauri dev```

## Build the thing

```npm run tauri build```