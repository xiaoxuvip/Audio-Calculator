use std::fs::File;
use std::path::Path;
use std::time::Duration;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

#[derive(Clone)]
pub struct AudioFileInfo {
    pub file_path: String,
    pub file_name: String,
    pub duration: Duration,
}

/// 获取音频文件时长
pub fn get_audio_duration(path: &str) -> Option<Duration> {
    // 首先尝试使用 Windows Shell API（与资源管理器一致）
    #[cfg(windows)]
    if let Some(duration) = get_shell_duration(path) {
        if duration > Duration::ZERO {
            return Some(duration);
        }
    }
    
    // 回退到 symphonia
    get_symphonia_duration(path)
}

/// 使用 symphonia 获取音频时长
fn get_symphonia_duration(path: &str) -> Option<Duration> {
    let file = File::open(path).ok()?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    
    let mut hint = Hint::new();
    if let Some(ext) = Path::new(path).extension() {
        hint.with_extension(&ext.to_string_lossy());
    }
    
    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();
    
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .ok()?;
    
    let format = probed.format;
    
    // 获取默认音轨
    let track = format.default_track()?;
    
    // 计算时长
    let time_base = track.codec_params.time_base?;
    let n_frames = track.codec_params.n_frames?;
    
    let duration_secs = (n_frames as f64 * time_base.numer as f64) / time_base.denom as f64;
    
    Some(Duration::from_secs_f64(duration_secs))
}

/// Windows Shell API 获取时长（与资源管理器显示一致）
#[cfg(windows)]
fn get_shell_duration(path: &str) -> Option<Duration> {
    use windows::core::PCWSTR;
    use windows::Win32::UI::Shell::PropertiesSystem::{
        IPropertyStore, PSGetPropertyKeyFromName, SHGetPropertyStoreFromParsingName,
        GPS_DEFAULT, PROPERTYKEY,
    };
    
    unsafe {
        // 转换路径为宽字符
        let wide_path: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
        
        // 获取属性存储
        let store: IPropertyStore = SHGetPropertyStoreFromParsingName(
            PCWSTR::from_raw(wide_path.as_ptr()),
            None,
            GPS_DEFAULT,
        ).ok()?;
        
        // 获取 System.Media.Duration 属性键
        let prop_name: Vec<u16> = "System.Media.Duration\0".encode_utf16().collect();
        let mut pkey = PROPERTYKEY::default();
        PSGetPropertyKeyFromName(PCWSTR::from_raw(prop_name.as_ptr()), &mut pkey).ok()?;
        
        // 获取属性值
        let pv = store.GetValue(&pkey).ok()?;
        
        // 时长以 100 纳秒为单位存储 (VT_UI8 = 21)
        // 使用 windows-rs 的 PROPVARIANT 类型
        if let Ok(duration_100ns) = i64::try_from(&pv) {
            if duration_100ns > 0 {
                return Some(Duration::from_nanos(duration_100ns as u64 * 100));
            }
        }
        
        None
    }
}

#[cfg(not(windows))]
fn get_shell_duration(_path: &str) -> Option<Duration> {
    None
}
