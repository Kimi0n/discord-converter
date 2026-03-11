pub fn calculate_bitrate(max_file_size_mb: f64, video_length_seconds: f64) -> f64 {
    if video_length_seconds <= 0.0 {
        return 0.0; //TODO: throw an error here instead of defaulting to 0
    }

    let safe_file_size_mb = max_file_size_mb * 0.95; // 5% margin for container overhead
    let total_kilobits = safe_file_size_mb * 8192.0;
    let bitrate_kbps = total_kilobits / video_length_seconds;

    bitrate_kbps
}

pub fn extract_resolution_number(quality_option: String) -> i32 {
    if quality_option == "Source" {
        0
    } else {
        quality_option
            .trim_end_matches('p')
            .parse::<i32>()
            .unwrap_or(0) //TODO: throw an error here instead of defaulting to source
    }
}

pub fn extract_framerate_number(framerate_option: String) -> i32 {
    if framerate_option == "Source" {
        0
    } else {
        framerate_option
            .parse::<i32>()
            .unwrap_or(0) //TODO: throw an error here instead of defaulting to source
    }
}

pub fn ffmpeg_time_to_seconds(time_in_ffmpeg_timestamp: &str) -> f64 {
    // Split by ':' and '.' to get [hh, mm, ss, ms]
    let parts: Vec<&str> = time_in_ffmpeg_timestamp.split(&[':', '.'][..]).collect();
    
    let hours: f64 = parts[0].parse().unwrap_or(0.0);
    let minutes: f64 = parts[1].parse().unwrap_or(0.0);
    let seconds: f64 = parts[2].parse().unwrap_or(0.0);
    let millis: f64 = parts.get(3).map_or(0.0, |ms| ms.parse().unwrap_or(0.0));

    // Calculation: (HH * 3600) + (MM * 60) + SS + (MS / 100)
    // Note: Use 1000.0 if your input represents true milliseconds (3 digits)
    (hours * 3600.0) + (minutes * 60.0) + seconds + (millis / 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_bitrate() {
        assert_eq!(calculate_bitrate(50.0, 0.0), 0.0);
        assert_eq!(calculate_bitrate(50.0, 12.0), 32426.666666666668);
    }

    #[test]
    fn test_extract_resolution_number() {
        assert_eq!(extract_resolution_number("Source".to_string()), 0);
        assert_eq!(extract_resolution_number("1080p".to_string()), 1080);
    }

    #[test]
    fn test_extract_framerate_number() {
        assert_eq!(extract_framerate_number("Source".to_string()), 0);
        assert_eq!(extract_framerate_number("60".to_string()), 60);
    }

    #[test]
    fn test_ffmpeg_time_to_seconds() {
        assert_eq!(ffmpeg_time_to_seconds("00:00:04.90"), 4.9);
        assert_eq!(ffmpeg_time_to_seconds("00:02:15.00"), 135.0);
        assert_eq!(ffmpeg_time_to_seconds("01:30:00.00"), 5400.0);
    }
}