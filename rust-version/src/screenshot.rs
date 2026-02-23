use crate::audio::AudioFileInfo;
use ab_glyph::{FontVec, PxScale};
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use std::time::Duration;

pub struct ImageData {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

pub fn generate_screenshot(
    files: &[AudioFileInfo],
    total_duration: Duration,
    price: &str,
    use_minute: bool,
) -> Result<ImageData, String> {
    let font = load_font().ok_or("无法加载字体")?;
    
    let width = 800u32;
    let file_list_height = calc_file_list_height(files.len(), 36);  // 行高36
    let height = (440 + file_list_height + 100) as u32;
    
    let mut img: RgbaImage = ImageBuffer::new(width, height);
    
    // 渐变背景: 暖橙白 -> 暖粉白
    for y in 0..height {
        let t = y as f32 / height as f32;
        let r = (255.0 - t * 1.0) as u8;
        let g = (247.0 - t * 5.0) as u8;
        let b = (237.0 + t * 5.0) as u8;
        for x in 0..width {
            img.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }
    
    // 白色卡片背景
    draw_filled_rect_mut(&mut img, Rect::at(40, 40).of_size(width - 80, height - 80), Rgba([255, 255, 255, 252]));
    
    // 顶部彩色条纹装饰
    for x in 40..width - 40 {
        let t = (x - 40) as f32 / (width - 80) as f32;
        let (r, g, b) = if t < 0.33 {
            (251, 146, 60)  // 橙色
        } else if t < 0.66 {
            (168, 85, 247)  // 紫色
        } else {
            (59, 130, 246)  // 蓝色
        };
        for y in 40..44 {
            img.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }
    
    // 标题 - 用文字代替emoji
    let title_color = Rgba([234u8, 88, 12, 255]);  // 橙色
    draw_text_mut(&mut img, title_color, 72, 70, PxScale::from(36.0), &font, "音频时长统计");
    
    let mut y = 130i32;
    
    // 信息栏 - 三个彩色卡片
    // 文件数 - 蓝色
    draw_filled_rect_mut(&mut img, Rect::at(72, y).of_size(200, 50), Rgba([239, 246, 255, 255]));
    draw_text_mut(&mut img, Rgba([59, 130, 246, 255]), 88, y + 14, PxScale::from(22.0), &font, 
        &format!("文件: {} 个", files.len()));
    
    // 时长 - 紫色
    draw_filled_rect_mut(&mut img, Rect::at(288, y).of_size(200, 50), Rgba([250, 245, 255, 255]));
    draw_text_mut(&mut img, Rgba([168, 85, 247, 255]), 304, y + 14, PxScale::from(22.0), &font, 
        &format!("时长: {}", format_duration(total_duration)));
    
    // 单价 - 绿色
    let unit = if use_minute { "分钟" } else { "小时" };
    draw_filled_rect_mut(&mut img, Rect::at(504, y).of_size(220, 50), Rgba([240, 253, 244, 255]));
    draw_text_mut(&mut img, Rgba([34, 197, 94, 255]), 520, y + 14, PxScale::from(22.0), &font, 
        &format!("单价: ¥{}/{}", price, unit));
    
    y += 70;
    
    // 文件列表
    if !files.is_empty() {
        draw_text_mut(&mut img, Rgba([107, 114, 128, 255]), 72, y, PxScale::from(24.0), &font, "文件列表:");
        y += 40;
        y = draw_file_list(&mut img, &font, files, 72, y, width - 144);
        y += 24;
    }
    
    // 总计金额
    draw_filled_rect_mut(&mut img, Rect::at(72, y).of_size(width - 144, 80), Rgba([255, 251, 235, 255]));
    draw_text_mut(&mut img, Rgba([107, 114, 128, 255]), 92, y + 12, PxScale::from(18.0), &font, "总计金额");
    
    let total_price = calculate_price(total_duration, price, use_minute);
    let price_str = format!("¥{:.2}", total_price);
    draw_text_mut(&mut img, Rgba([234, 88, 12, 255]), 92, y + 38, PxScale::from(42.0), &font, &price_str);
    
    // 时间戳
    let now = chrono::Local::now();
    let time_str = now.format("%Y-%m-%d %H:%M").to_string();
    draw_text_mut(&mut img, Rgba([156, 163, 175, 255]), 72, (height - 68) as i32, PxScale::from(16.0), &font, &time_str);
    
    Ok(ImageData {
        width: width as usize,
        height: height as usize,
        data: img.into_raw(),
    })
}

fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn calculate_price(total_duration: Duration, price: &str, use_minute: bool) -> f64 {
    let price: f64 = price.parse().unwrap_or(0.0);
    if use_minute { total_duration.as_secs_f64() / 60.0 * price }
    else { total_duration.as_secs_f64() / 3600.0 * price }
}

fn load_font() -> Option<FontVec> {
    let font_paths = [
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\msyh.ttf", 
        "C:\\Windows\\Fonts\\simhei.ttf",
        "C:\\Windows\\Fonts\\simsun.ttc",
    ];
    for path in &font_paths {
        if let Ok(data) = std::fs::read(path) {
            if let Ok(font) = FontVec::try_from_vec(data) {
                return Some(font);
            }
        }
    }
    None
}

fn calc_file_list_height(file_count: usize, line_height: i32) -> i32 {
    if file_count == 0 { return 0; }
    let cols = if file_count > 10 { 2 } else { 1 };
    let rows = ((file_count as f64) / (cols as f64)).ceil() as i32;
    rows * line_height + 20
}

fn draw_file_list(img: &mut RgbaImage, font: &FontVec, files: &[AudioFileInfo], x: i32, y: i32, total_width: u32) -> i32 {
    if files.is_empty() { return y; }
    
    let cols = if files.len() > 10 { 2 } else { 1 };
    let line_height = 36i32;  // 增大行高
    let text_color = Rgba([75u8, 85, 99, 255]);
    let duration_color = Rgba([234u8, 88, 12, 255]);
    // 交替行背景色
    let row_bg_even = Rgba([248u8, 250, 252, 255]);  // 浅灰
    let row_bg_odd = Rgba([255u8, 255, 255, 255]);   // 白色
    
    if cols == 1 {
        for (i, file) in files.iter().enumerate() {
            let row_y = y + i as i32 * line_height;
            // 绘制交替行背景
            let bg_color = if i % 2 == 0 { row_bg_even } else { row_bg_odd };
            draw_filled_rect_mut(img, Rect::at(x - 8, row_y - 2).of_size(total_width + 16, line_height as u32), bg_color);
            
            let mut name = file.file_name.clone();
            if name.chars().count() > 28 {
                name = name.chars().take(25).collect::<String>() + "...";
            }
            let text = format!("* {}", name);
            draw_text_mut(img, text_color, x, row_y + 4, PxScale::from(20.0), font, &text);
            
            let duration_text = format_duration(file.duration);
            draw_text_mut(img, duration_color, x + total_width as i32 - 90, row_y + 4, 
                PxScale::from(18.0), font, &duration_text);
        }
        return y + files.len() as i32 * line_height;
    } else {
        let col_width = (total_width / 2 - 16) as i32;
        let items_per_col = ((files.len() as f64) / 2.0).ceil() as usize;
        
        for (i, file) in files.iter().enumerate() {
            let col = i / items_per_col;
            let row = i % items_per_col;
            let item_x = x + col as i32 * (col_width + 32);
            let item_y = y + row as i32 * line_height;
            
            // 绘制交替行背景
            let bg_color = if row % 2 == 0 { row_bg_even } else { row_bg_odd };
            draw_filled_rect_mut(img, Rect::at(item_x - 8, item_y - 2).of_size(col_width as u32 + 16, line_height as u32), bg_color);
            
            let mut name = file.file_name.clone();
            if name.chars().count() > 12 {
                name = name.chars().take(9).collect::<String>() + "...";
            }
            let text = format!("* {} ({})", name, format_duration(file.duration));
            draw_text_mut(img, text_color, item_x, item_y + 4, PxScale::from(18.0), font, &text);
        }
        
        let rows = ((files.len() as f64) / 2.0).ceil() as i32;
        return y + rows * line_height;
    }
}
