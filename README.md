# Discord Converter

![Preview image of the app](src/assets/app_preview.png "Preview Image")

A video converter that compresses your video to a specific filesize so it can be sent over Discord (or anything else really). Windows only.

# Download

Go to [this link](https://github.com/Kimi0n/discord-converter/releases) and download and extract the ZIP file. Run the exe.

# How to build (devs only)

1. Pull the repository.
2. Ensure Rust and npm are installed on your device.
3. Download and add an ffmpeg binary to `src-tauri/bin` and rename it to:

`ffmpeg-x86_64-pc-windows-msvc.exe`

## Start dev server

```sh
npm run tauri dev
```

## Test the thing (backend only for now)

```sh
npm test
```

## Build the thing

```sh
npm run tauri build
```

# Features i want to implement for v1.0

- Subtitle hardsubbing with the subtitles video filter
- HDR to SDR tonemapping if the video is HDR
- Abort button for conversion
- Config file that remembers your last settings
- Hardware accelerated encoding for AMD and Intel (figure out where to get that info from)
- Write tests for frontend and backend and implement proper logging
- Create a custom icon for the program
- Design a pretty UI