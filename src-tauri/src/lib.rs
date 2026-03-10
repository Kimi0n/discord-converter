mod video_helper_functions;

use std::path::Path;
use tauri_plugin_shell::ShellExt;
use ffprobe::ffprobe;

struct FileInfo {
    path: String,
    file_name: String,
    extension: String,
    parent_path: String
}

#[tauri::command]
async fn convert_video(app: tauri::AppHandle, file_path: String, max_file_size: f64,
    quality_option: String, framerate_option: String, is_hardware_accelerated: bool,
    is_modern_codec: bool) -> Result<String, String> {
    // println!("Backend received path: {}", file_path);

    let file_data: FileInfo = split_filepath(&file_path);
    let video_length_seconds: f64 = call_ffprobe_get_video_length(&file_path);
    let bitrate_in_kbps = video_helper_functions::calculate_bitrate(max_file_size, video_length_seconds);
    let selected_resolution = video_helper_functions::extract_resolution_number(quality_option);
    let selected_framerate = video_helper_functions::extract_framerate_number(framerate_option);

    call_ffmpeg_for_conversion(app, file_data, bitrate_in_kbps, max_file_size,
        selected_resolution, selected_framerate, is_hardware_accelerated, is_modern_codec).await
}

fn call_ffprobe_get_video_length(file_path_string: &String) -> f64 {
    let result = ffprobe(file_path_string).expect("Failed to probe file");

    if let Some(duration_str) = result.format.duration {
        duration_str.parse().unwrap_or(0.0)
    } else {
        0.0 //TODO: throw an error here instead of defaulting to 0
    }
}

fn split_filepath(file_path_string: &String) -> FileInfo {
    let path = Path::new(&file_path_string);

    // Get the extension (e.g., "pdf")
    let file_extension = path.extension()
        .and_then(|os_str| os_str.to_str());

    // Get the filename without the extension (e.g., "my_document")
    let file_stem = path.file_stem()
        .and_then(|os_str| os_str.to_str());

    // Get the filename without the extension (e.g., "my_document")
    let parent_path = path.parent()
        .and_then(|os_str| os_str.to_str());

    FileInfo {
        file_name: file_stem.unwrap_or("N/A").to_string(),
        extension: file_extension.unwrap_or("N/A").to_string(),
        parent_path: parent_path.unwrap_or("N/A").to_string(),
        path: file_path_string.clone()
    }
}

async fn call_ffmpeg_for_conversion(app: tauri::AppHandle, video_file_info: FileInfo,
    bitrate_in_kbps: f64, max_file_size: f64, selected_resolution: i32, selected_framerate: i32, 
    is_hardware_accelerated: bool, is_modern_codec: bool) -> Result<String, String> {

    let output_path = format!("{}\\{}-{}M.{}", video_file_info.parent_path, video_file_info.file_name, max_file_size, video_file_info.extension);
    let audio_bitrate_kbps: f64 = 128.0;
    let audio_adjusted_bitrate: f64 = bitrate_in_kbps - audio_bitrate_kbps;

    let bitrate_str = format!("{}k", audio_adjusted_bitrate);
    let bufsize_str = format!("{}k", audio_adjusted_bitrate * 2.0);
    let audio_bitrate_str = format!("{}k", audio_bitrate_kbps);

    let mut selected_codec: String = "libx264".to_string();
    let mut selected_preset: String = "slow".to_string();

    if is_modern_codec && !is_hardware_accelerated {
        selected_codec = "libx265".to_string();
    }

    if !is_modern_codec && is_hardware_accelerated {
        selected_codec = "h264_nvenc".to_string();
        selected_preset = "p7".to_string();
    }

    if is_modern_codec && is_hardware_accelerated {
        selected_codec = "hevc_nvenc".to_string();
        selected_preset = "p7".to_string();
    }

    let mut ffmpeg_args = vec![
        "-y".to_string(), 
        "-i".to_string(), video_file_info.path, 
        "-c:v".to_string(), selected_codec,
        "-preset".to_string(), selected_preset,
        "-b:v".to_string(), bitrate_str.clone(),
        "-maxrate".to_string(), bitrate_str,
        "-bufsize".to_string(), bufsize_str,
    ];

    if selected_resolution > 0 {
        ffmpeg_args.push("-vf".to_string());
        ffmpeg_args.push(format!("scale=-2:{}", selected_resolution));
    }

    if selected_framerate > 0 {
        ffmpeg_args.push("-r".to_string());
        ffmpeg_args.push(selected_framerate.to_string());
    }

    ffmpeg_args.extend([
        "-c:a".to_string(), "aac".to_string(),
        "-b:a".to_string(), audio_bitrate_str, 
        output_path
    ]);

    // println!("{:#?}", ffmpeg_args);

    let ffmpeg_command = app.shell()
        .sidecar("ffmpeg") 
        .unwrap()
        .args(ffmpeg_args);

    let output = ffmpeg_command.output().await.expect("failed to execute ffmpeg");
    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        let possible_errors = ["required", "Error", "Failed"];
        let specific_error = error_message
            .lines()
            .find(|line| possible_errors.iter().any(|&word| line.contains(word)))
            .unwrap_or("Unknown FFmpeg error");

        return Err(specific_error.to_string());
    }

    Ok("Success!".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![convert_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
