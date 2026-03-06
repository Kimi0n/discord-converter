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
async fn convert_video(app: tauri::AppHandle, file_path: String, max_file_size: f64) {
    // println!("Backend received path: {}", file_path);

    let file_data: FileInfo = split_filepath(&file_path);
    let video_length_seconds: f64 = call_ffprobe_get_video_length(&file_path);
    let bitrate_in_kbps = calculate_bitrate(max_file_size, video_length_seconds);
    call_ffmpeg_for_conversion(app, file_data, bitrate_in_kbps, max_file_size).await;
}

fn call_ffprobe_get_video_length(file_path_string: &String) -> f64 {
    let result = ffprobe(file_path_string).expect("Failed to probe file");

    if let Some(duration_str) = result.format.duration {
        duration_str.parse().unwrap_or(0.0)
    } else {
        0.0
    }
}

fn calculate_bitrate(max_file_size_mb: f64, video_length_seconds: f64) -> f64 {
    if video_length_seconds <= 0.0 {
        return 0.0;
    }

    // Apply a 5% margin to account for container overhead
    let safe_file_size_mb = max_file_size_mb * 0.95;
    let total_kilobits = safe_file_size_mb * 8192.0;
    let bitrate_kbps = total_kilobits / video_length_seconds;

    bitrate_kbps
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

async fn call_ffmpeg_for_conversion(app: tauri::AppHandle, video_file_info: FileInfo, bitrate_in_kbps: f64, max_file_size: f64) {
    let output_path = format!("{}\\{}-{}M.{}", video_file_info.parent_path, video_file_info.file_name, max_file_size, video_file_info.extension);
    let audio_bitrate_kbps: f64 = 128.0;
    let audio_adjusted_bitrate: f64 = bitrate_in_kbps - audio_bitrate_kbps;

    let bitrate_str = format!("{}k", audio_adjusted_bitrate);
    let bufsize_str = format!("{}k", audio_adjusted_bitrate * 2.0);
    let audio_bitrate_str = format!("{}k", audio_bitrate_kbps);
    // println!("Bitrate: {}, Bufsize: {}", bitrate_str, bufsize_str);

    let sidecar_command = app.shell()
        .sidecar("ffmpeg") // Note: use the base name only
        .unwrap()
        .args([
            "-y", 
            "-i", &video_file_info.path, 
            "-c:v", "libx264",
            "-preset", "slow",
            "-b:v", &bitrate_str,
            "-maxrate", &bitrate_str,
            "-bufsize", &bufsize_str,
            "-c:a", "aac",
            "-b:a", &audio_bitrate_str, 
            &output_path
        ]);

    let output = sidecar_command.output().await.unwrap();
    if output.status.success() {
        // println!("Success: {}", output_path);
    }
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
