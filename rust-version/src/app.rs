use crate::audio::{AudioFileInfo, get_audio_duration};
use crate::screenshot::generate_screenshot;
use crate::shell::{add_context_menu, remove_context_menu};
use arboard::Clipboard;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use rfd::FileDialog;
use std::path::Path;
use std::time::Duration;
use walkdir::WalkDir;

const SUPPORTED_EXTENSIONS: &[&str] = &[
    "mp3", "wav", "flac", "m4a", "ogg", "wma", "aac", 
    "aiff", "ape", "opus", "webm", "mp4", "mkv"
];

// 全新现代配色方案 - 明亮活泼
const BG_COLOR: Color32 = Color32::from_rgb(241, 245, 249);           // 轻微蓝灰色调 #F1F5F9 (蓝多一点)
const CARD_COLOR: Color32 = Color32::WHITE;
const TEXT_PRIMARY: Color32 = Color32::from_rgb(31, 41, 55);          // 深灰 #1F2937
const TEXT_SECONDARY: Color32 = Color32::from_rgb(107, 114, 128);     // 中灰 #6B7280

// 主题色 - 活力橙
const ACCENT_ORANGE: Color32 = Color32::from_rgb(251, 146, 60);       // #FB923C
const ACCENT_ORANGE_DARK: Color32 = Color32::from_rgb(234, 88, 12);   // #EA580C

// 功能色
const SUCCESS_GREEN: Color32 = Color32::from_rgb(34, 197, 94);        // #22C55E
const INFO_BLUE: Color32 = Color32::from_rgb(59, 130, 246);           // #3B82F6
const PURPLE_ACCENT: Color32 = Color32::from_rgb(168, 85, 247);       // #A855F7
const PINK_ACCENT: Color32 = Color32::from_rgb(236, 72, 153);         // #EC4899
const CYAN_ACCENT: Color32 = Color32::from_rgb(6, 182, 212);          // #06B6D4

// 边框和分隔
const BORDER_LIGHT: Color32 = Color32::from_rgb(243, 244, 246);       // #F3F4F6
const BORDER_MEDIUM: Color32 = Color32::from_rgb(229, 231, 235);      // #E5E7EB

pub struct AudioCalculatorApp {
    audio_files: Vec<AudioFileInfo>,
    total_duration: Duration,
    price_input: String,
    use_minute: bool,
    message: Option<(String, bool)>,
}

impl AudioCalculatorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, args: Vec<String>) -> Self {
        let mut app = Self {
            audio_files: Vec::new(),
            total_duration: Duration::ZERO,
            price_input: "100".to_string(),
            use_minute: false,
            message: None,
        };
        if !args.is_empty() { app.add_files_from_args(&args); }
        app
    }

    fn add_files_from_args(&mut self, args: &[String]) {
        let mut files = Vec::new();
        for arg in args {
            let path = Path::new(arg);
            if path.is_dir() {
                for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                    if entry.file_type().is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if SUPPORTED_EXTENSIONS.contains(&ext.to_string_lossy().to_lowercase().as_str()) {
                                files.push(entry.path().to_path_buf());
                            }
                        }
                    }
                }
            } else if path.is_file() { files.push(path.to_path_buf()); }
        }
        self.add_files(files.iter().map(|p| p.to_string_lossy().to_string()).collect());
    }

    fn add_files(&mut self, file_paths: Vec<String>) {
        let mut new_files = Vec::new();
        for path in file_paths {
            let path_obj = Path::new(&path);
            if !path_obj.exists() { continue; }
            let ext = path_obj.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
            if !SUPPORTED_EXTENSIONS.contains(&ext.as_str()) { continue; }
            if self.audio_files.iter().any(|f| f.file_path == path) { continue; }
            if new_files.iter().any(|f: &AudioFileInfo| f.file_path == path) { continue; }
            if let Some(duration) = get_audio_duration(&path) {
                let file_name = path_obj.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                new_files.push(AudioFileInfo { file_path: path, file_name, duration });
            }
        }
        new_files.sort_by(|a, b| a.file_name.to_lowercase().cmp(&b.file_name.to_lowercase()));
        for f in new_files { self.audio_files.push(f); }
        self.audio_files.sort_by(|a, b| a.file_name.to_lowercase().cmp(&b.file_name.to_lowercase()));
        self.update_total_duration();
    }

    fn update_total_duration(&mut self) {
        self.total_duration = self.audio_files.iter().map(|f| f.duration).sum();
    }

    fn calculate_price(&self) -> f64 {
        let price: f64 = self.price_input.parse().unwrap_or(0.0);
        if self.use_minute { self.total_duration.as_secs_f64() / 60.0 * price }
        else { self.total_duration.as_secs_f64() / 3600.0 * price }
    }

    fn format_duration(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    fn clear_files(&mut self) {
        self.audio_files.clear();
        self.total_duration = Duration::ZERO;
    }

    fn select_files(&mut self) {
        if let Some(paths) = FileDialog::new()
            .add_filter("音频文件", &["mp3", "wav", "flac", "m4a", "ogg", "wma", "aac", "aiff", "ape", "opus"])
            .add_filter("所有文件", &["*"])
            .set_title("选择音频文件")
            .pick_files()
        {
            self.add_files(paths.iter().map(|p| p.to_string_lossy().to_string()).collect());
        }
    }

    fn select_folder(&mut self) {
        if let Some(path) = FileDialog::new().set_title("选择包含音频文件的文件夹").pick_folder() {
            let mut files = Vec::new();
            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    if let Some(ext) = entry.path().extension() {
                        if SUPPORTED_EXTENSIONS.contains(&ext.to_string_lossy().to_lowercase().as_str()) {
                            files.push(entry.path().to_string_lossy().to_string());
                        }
                    }
                }
            }
            self.add_files(files);
        }
    }

    fn copy_screenshot(&mut self) {
        match generate_screenshot(&self.audio_files, self.total_duration, &self.price_input, self.use_minute) {
            Ok(image_data) => {
                if let Ok(mut clipboard) = Clipboard::new() {
                    let img = arboard::ImageData {
                        width: image_data.width,
                        height: image_data.height,
                        bytes: std::borrow::Cow::Owned(image_data.data),
                    };
                    if clipboard.set_image(img).is_ok() {
                        self.message = Some(("截图已复制到剪贴板！".to_string(), true));
                    } else {
                        self.message = Some(("复制到剪贴板失败".to_string(), false));
                    }
                }
            }
            Err(e) => { self.message = Some((format!("截图失败: {}", e), false)); }
        }
    }
}

impl eframe::App for AudioCalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                let files: Vec<String> = i.raw.dropped_files.iter()
                    .filter_map(|f| f.path.as_ref())
                    .map(|p| p.to_string_lossy().to_string())
                    .collect();
                if !files.is_empty() { self.add_files_from_args(&files); }
            }
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(BG_COLOR))
            .show(ctx, |ui| {
                self.render_header(ui);
                egui::Frame::none().inner_margin(egui::Margin::same(16.0)).show(ui, |ui| {
                    self.render_file_list(ui);
                    ui.add_space(10.0);
                    self.render_stats(ui);
                    ui.add_space(8.0);
                    self.render_price_settings(ui);
                    ui.add_space(10.0);
                    self.render_buttons(ui);
                });
            });

        if let Some((msg, is_success)) = self.message.take() {
            rfd::MessageDialog::new()
                .set_title(if is_success { "成功" } else { "错误" })
                .set_description(&msg)
                .set_level(if is_success { rfd::MessageLevel::Info } else { rfd::MessageLevel::Error })
                .show();
        }
    }
}


impl AudioCalculatorApp {
    fn render_header(&mut self, ui: &mut egui::Ui) {
        let header_height = 80.0;
        let full_width = ui.available_width();
        let (rect, _) = ui.allocate_exact_size(Vec2::new(full_width, header_height), egui::Sense::hover());
        let painter = ui.painter_at(rect);
        
        // 横向渐变：左侧蓝色 #3B82F6 -> 右侧粉色 #EC4899
        let mut mesh = egui::Mesh::default();
        let color_left = Color32::from_rgb(59, 130, 246);   // 蓝色（原右上）
        let color_right = Color32::from_rgb(236, 72, 153);  // 粉色（原左下）
        
        mesh.colored_vertex(rect.left_top(), color_left);
        mesh.colored_vertex(rect.right_top(), color_right);
        mesh.colored_vertex(rect.right_bottom(), color_right);
        mesh.colored_vertex(rect.left_bottom(), color_left);
        mesh.add_triangle(0, 1, 2);
        mesh.add_triangle(0, 2, 3);
        painter.add(egui::Shape::mesh(mesh));
        
        // Logo - 白色圆角方形背景
        let logo_rect = egui::Rect::from_min_size(rect.min + Vec2::new(16.0, 16.0), Vec2::new(48.0, 48.0));
        painter.rect_filled(logo_rect, Rounding::same(12.0), Color32::from_rgba_unmultiplied(255, 255, 255, 230));
        painter.text(logo_rect.center(), egui::Align2::CENTER_CENTER, "🎵", egui::FontId::proportional(26.0), Color32::from_rgb(236, 72, 153));
        
        let logo_response = ui.interact(logo_rect, ui.id().with("logo"), egui::Sense::click());
        if logo_response.clicked() { let _ = open::that("https://www.xiaoxu.vip/ypsc"); }
        if logo_response.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
        
        // 标题
        painter.text(rect.min + Vec2::new(76.0, 32.0), egui::Align2::LEFT_CENTER, "音频时长统计", 
            egui::FontId::proportional(20.0), Color32::WHITE);
        painter.text(rect.min + Vec2::new(76.0, 56.0), egui::Align2::LEFT_CENTER, "👈点击图标获取最新版", 
            egui::FontId::proportional(11.0), Color32::from_rgba_unmultiplied(255, 255, 255, 180));
        
        // 右侧按钮
        let btn_y = rect.center().y;
        
        // 添加右键按钮 - 半透明白色背景带阴影
        let add_rect = egui::Rect::from_center_size(egui::pos2(rect.right() - 152.0, btn_y), Vec2::new(80.0, 32.0));
        let add_resp = ui.interact(add_rect, ui.id().with("add_btn"), egui::Sense::click());
        let add_bg = if add_resp.hovered() { Color32::WHITE } else { Color32::from_rgba_unmultiplied(255, 255, 255, 200) };
        // 按钮阴影
        painter.rect_filled(
            add_rect.translate(Vec2::new(0.0, 2.0)),
            Rounding::same(8.0),
            Color32::from_rgba_unmultiplied(0, 0, 0, 30)
        );
        painter.rect_filled(add_rect, Rounding::same(8.0), add_bg);
        painter.text(add_rect.center(), egui::Align2::CENTER_CENTER, "+ 添加右键", 
            egui::FontId::proportional(11.0), Color32::from_rgb(59, 130, 246));
        if add_resp.clicked() {
            match add_context_menu() {
                Ok(_) => self.message = Some(("已添加到右键菜单！".to_string(), true)),
                Err(e) => self.message = Some((e, false)),
            }
        }
        
        // 移除右键按钮 - 半透明白色背景带阴影
        let remove_rect = egui::Rect::from_center_size(egui::pos2(rect.right() - 56.0, btn_y), Vec2::new(80.0, 32.0));
        let remove_resp = ui.interact(remove_rect, ui.id().with("remove_btn"), egui::Sense::click());
        let remove_bg = if remove_resp.hovered() { Color32::WHITE } else { Color32::from_rgba_unmultiplied(255, 255, 255, 200) };
        // 按钮阴影
        painter.rect_filled(
            remove_rect.translate(Vec2::new(0.0, 2.0)),
            Rounding::same(8.0),
            Color32::from_rgba_unmultiplied(0, 0, 0, 30)
        );
        painter.rect_filled(remove_rect, Rounding::same(8.0), remove_bg);
        painter.text(remove_rect.center(), egui::Align2::CENTER_CENTER, "- 移除右键", 
            egui::FontId::proportional(11.0), Color32::from_rgb(236, 72, 153));
        if remove_resp.clicked() {
            match remove_context_menu() {
                Ok(_) => self.message = Some(("已移除右键菜单！".to_string(), true)),
                Err(e) => self.message = Some((e, false)),
            }
        }
    }

    fn render_file_list(&mut self, ui: &mut egui::Ui) {
        let available_height = 298.0;  // 总高度298，内容区域250
        let full_width = ui.available_width();
        
        egui::Frame::none()
            .fill(CARD_COLOR)
            .rounding(Rounding::same(16.0))
            .stroke(Stroke::new(1.0, BORDER_MEDIUM))
            .shadow(egui::epaint::Shadow {
                offset: Vec2::new(0.0, 2.0),
                blur: 8.0,
                spread: 2.0,
                color: Color32::from_rgba_unmultiplied(0, 0, 0, 20),
            })
            .show(ui, |ui| {
                ui.set_width(full_width);
                
                // 标题栏 - 与背景色一致
                let header_rect = ui.allocate_space(Vec2::new(full_width, 48.0)).1;
                let painter = ui.painter();
                painter.rect_filled(
                    header_rect,
                    Rounding { nw: 16.0, ne: 16.0, sw: 0.0, se: 0.0 },
                    BG_COLOR
                );
                
                // 标题内容
                painter.text(header_rect.left_center() + Vec2::new(16.0, 0.0), egui::Align2::LEFT_CENTER,
                    "📁", egui::FontId::proportional(16.0), ACCENT_ORANGE);
                painter.text(header_rect.left_center() + Vec2::new(40.0, 0.0), egui::Align2::LEFT_CENTER,
                    "文件列表", egui::FontId::proportional(14.0), TEXT_PRIMARY);
                painter.text(header_rect.left_center() + Vec2::new(100.0, 0.0), egui::Align2::LEFT_CENTER,
                    &format!("({} 个)", self.audio_files.len()), egui::FontId::proportional(12.0), TEXT_SECONDARY);
                
                // 清空按钮
                let clear_rect = egui::Rect::from_center_size(
                    egui::pos2(header_rect.right() - 40.0, header_rect.center().y), Vec2::new(50.0, 26.0));
                let clear_resp = ui.interact(clear_rect, ui.id().with("clear_btn"), egui::Sense::click());
                let clear_bg = if clear_resp.hovered() { Color32::from_rgb(254, 226, 226) } else { CARD_COLOR };
                painter.rect(clear_rect, Rounding::same(6.0), clear_bg, Stroke::new(1.0, BORDER_MEDIUM));
                painter.text(clear_rect.center(), egui::Align2::CENTER_CENTER, "清空", 
                    egui::FontId::proportional(11.0), if clear_resp.hovered() { PINK_ACCENT } else { TEXT_SECONDARY });
                if clear_resp.clicked() { self.clear_files(); }
                
                let content_height = available_height - 48.0;
                
                if self.audio_files.is_empty() {
                    // 空状态 - 相对于内容区域上下左右居中（使用painter直接绘制）
                    let (content_rect, _) = ui.allocate_exact_size(Vec2::new(full_width, content_height), egui::Sense::hover());
                    let painter = ui.painter();
                    let center = content_rect.center();
                    
                    // 渐变圆形背景
                    painter.circle_filled(center + Vec2::new(0.0, -30.0), 36.0, Color32::from_rgb(255, 237, 213));
                    painter.circle_filled(center + Vec2::new(0.0, -30.0), 28.0, Color32::from_rgb(254, 215, 170));
                    painter.text(center + Vec2::new(0.0, -30.0), egui::Align2::CENTER_CENTER, "📂", 
                        egui::FontId::proportional(32.0), Color32::WHITE);
                    
                    // 文字
                    painter.text(center + Vec2::new(0.0, 26.0), egui::Align2::CENTER_CENTER, 
                        "拖拽音频文件到此处", egui::FontId::proportional(14.0), TEXT_PRIMARY);
                    painter.text(center + Vec2::new(0.0, 50.0), egui::Align2::CENTER_CENTER, 
                        "或点击下方按钮选择文件", egui::FontId::proportional(12.0), TEXT_SECONDARY);
                } else {
                    egui::ScrollArea::vertical().max_height(content_height).show(ui, |ui| {
                        ui.add_space(8.0);
                        egui::Frame::none().inner_margin(egui::Margin::symmetric(8.0, 0.0)).show(ui, |ui| {
                            for (i, file) in self.audio_files.iter().enumerate() {
                                let bg_color = if i % 2 == 0 { Color32::from_rgb(255, 255, 255) } 
                                              else { Color32::from_rgb(243, 244, 246) };  // 更明显的灰色
                                egui::Frame::none()
                                    .fill(bg_color)
                                    .rounding(Rounding::same(10.0))
                                    .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.set_min_width(ui.available_width());
                                            ui.label(RichText::new("🎵").size(14.0));
                                            ui.add_space(6.0);
                                            ui.add(egui::Label::new(RichText::new(&file.file_name).size(12.0).color(TEXT_PRIMARY)).truncate());
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                egui::Frame::none()
                                                    .fill(Color32::from_rgb(254, 243, 199))
                                                    .rounding(Rounding::same(6.0))
                                                    .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                                    .show(ui, |ui| {
                                                        ui.label(RichText::new(Self::format_duration(file.duration))
                                                            .size(11.0).color(ACCENT_ORANGE_DARK));
                                                    });
                                            });
                                        });
                                    });
                                ui.add_space(4.0);
                            }
                        });
                        ui.add_space(8.0);
                    });
                }
            });
    }


    fn render_stats(&self, ui: &mut egui::Ui) {
        let full_width = ui.available_width();
        let card_height = 52.0;
        
        egui::Frame::none()
            .fill(CARD_COLOR)
            .rounding(Rounding::same(12.0))
            .stroke(Stroke::new(1.0, BORDER_LIGHT))
            .show(ui, |ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::new(full_width, card_height), egui::Sense::hover());
                let painter = ui.painter();
                
                // 垂直居中的 Y 坐标
                let center_y = rect.center().y;
                
                // 计算内容布局 - 水平居中
                // 时长: 00:00:00    金额: ¥0.00
                let duration_text = Self::format_duration(self.total_duration);
                let price_text = format!("¥{:.2}", self.calculate_price());
                
                // 起始 X 位置（居中布局）
                let start_x = rect.left() + 32.0;
                
                // "时长:" 标签 - 14px 字体，与大字体底部对齐
                // 大字体22px，小字体14px，差值8px，小字体需要往下偏移约2px来对齐底部
                painter.text(
                    egui::pos2(start_x, center_y + 2.0),
                    egui::Align2::LEFT_CENTER,
                    "时长:",
                    egui::FontId::proportional(14.0),
                    TEXT_SECONDARY,
                );
                
                // 时长数值 - 22px 字体
                painter.text(
                    egui::pos2(start_x + 48.0, center_y),
                    egui::Align2::LEFT_CENTER,
                    &duration_text,
                    egui::FontId::proportional(22.0),
                    TEXT_PRIMARY,
                );
                
                // "金额:" 标签
                let price_label_x = start_x + 190.0;
                painter.text(
                    egui::pos2(price_label_x, center_y + 2.0),
                    egui::Align2::LEFT_CENTER,
                    "金额:",
                    egui::FontId::proportional(14.0),
                    TEXT_SECONDARY,
                );
                
                // 金额数值 - 22px 字体
                painter.text(
                    egui::pos2(price_label_x + 48.0, center_y),
                    egui::Align2::LEFT_CENTER,
                    &price_text,
                    egui::FontId::proportional(22.0),
                    SUCCESS_GREEN,
                );
            });
    }

    fn render_price_settings(&mut self, ui: &mut egui::Ui) {
        let full_width = ui.available_width();
        
        // 单选按钮颜色常量
        let selected_color = PURPLE_ACCENT;  // 选中：紫色
        let unselected_color = Color32::from_rgb(200, 200, 200);  // 未选中：浅灰色
        
        egui::Frame::none()
            .fill(CARD_COLOR)
            .rounding(Rounding::same(12.0))
            .stroke(Stroke::new(1.0, BORDER_LIGHT))
            .inner_margin(egui::Margin::symmetric(16.0, 12.0))
            .show(ui, |ui| {
                ui.set_width(full_width - 32.0);
                
                // 内容整体居中 - 使用 centered_and_justified
                ui.horizontal(|ui| {
                    // 计算内容总宽度并居中
                    let content_width = 280.0;  // 估算内容宽度
                    let padding = (ui.available_width() - content_width) / 2.0;
                    if padding > 0.0 {
                        ui.add_space(padding);
                    }
                    
                    // 单价标签 - 垂直居中
                    ui.vertical(|ui| {
                        ui.add_space(8.0);
                        ui.label(RichText::new("单价:").size(14.0).color(TEXT_PRIMARY));
                    });
                    ui.add_space(10.0);
                    
                    // 输入框 - 白色背景，黑色文字，内容居中
                    egui::Frame::none()
                        .fill(Color32::WHITE)
                        .stroke(Stroke::new(1.0, BORDER_MEDIUM))
                        .rounding(Rounding::same(6.0))
                        .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                        .show(ui, |ui| {
                            let text_edit = egui::TextEdit::singleline(&mut self.price_input)
                                .desired_width(50.0)
                                .horizontal_align(egui::Align::Center)
                                .font(egui::FontId::proportional(14.0))
                                .text_color(Color32::BLACK)
                                .frame(false);
                            ui.add(text_edit);
                        });
                    
                    ui.add_space(6.0);
                    
                    // 元/ - 垂直居中
                    ui.vertical(|ui| {
                        ui.add_space(8.0);
                        ui.label(RichText::new("元/").size(14.0).color(TEXT_PRIMARY));
                    });
                    ui.add_space(12.0);
                    
                    // Radio按钮 - 手动绘制以确保颜色正确
                    // 小时选项
                    let hour_selected = !self.use_minute;
                    let hour_color = if hour_selected { selected_color } else { unselected_color };
                    let hour_resp = ui.horizontal(|ui| {
                        let (rect, response) = ui.allocate_exact_size(Vec2::new(16.0, 16.0), egui::Sense::click());
                        let painter = ui.painter();
                        painter.circle_stroke(rect.center(), 7.0, Stroke::new(2.0, hour_color));
                        if hour_selected {
                            painter.circle_filled(rect.center(), 4.0, hour_color);
                        }
                        ui.add_space(4.0);
                        ui.label(RichText::new("小时").size(13.0).color(TEXT_PRIMARY));
                        response
                    }).inner;
                    if hour_resp.clicked() { self.use_minute = false; }
                    
                    ui.add_space(16.0);
                    
                    // 分钟选项
                    let minute_selected = self.use_minute;
                    let minute_color = if minute_selected { selected_color } else { unselected_color };
                    let minute_resp = ui.horizontal(|ui| {
                        let (rect, response) = ui.allocate_exact_size(Vec2::new(16.0, 16.0), egui::Sense::click());
                        let painter = ui.painter();
                        painter.circle_stroke(rect.center(), 7.0, Stroke::new(2.0, minute_color));
                        if minute_selected {
                            painter.circle_filled(rect.center(), 4.0, minute_color);
                        }
                        ui.add_space(4.0);
                        ui.label(RichText::new("分钟").size(13.0).color(TEXT_PRIMARY));
                        response
                    }).inner;
                    if minute_resp.clicked() { self.use_minute = true; }
                });
            });
    }

    fn render_buttons(&mut self, ui: &mut egui::Ui) {
        let full_width = ui.available_width();
        // 严格执行：左按钮距边框16px，右按钮距边框16px
        // 外层Frame已有16px边距，所以这里的0就是距离边框16px
        // 可用宽度 = full_width，需要在这个宽度内放置3个按钮
        // 按钮区域宽度 = full_width (因为外层已有边距)
        // 3个按钮 + 2个间距 = full_width
        // 设间距为固定值，按钮宽度动态计算
        let btn_count = 3.0;
        let spacing_count = 2.0;
        let spacing = 12.0;  // 按钮间距12px
        let btn_width = (full_width - spacing_count * spacing) / btn_count;
        let btn_height = 40.0;
        
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;  // 禁用默认间距，手动控制
            
            // 文件按钮 - 紧贴左边（外层已有16px边距）
            if ui.add_sized([btn_width, btn_height], egui::Button::new(RichText::new("📂 文件").size(13.0).color(TEXT_PRIMARY))
                .fill(CARD_COLOR)
                .stroke(Stroke::new(1.0, BORDER_MEDIUM))
                .rounding(Rounding::same(10.0))).clicked() {
                self.select_files();
            }
            ui.add_space(spacing);
            
            // 文件夹按钮
            if ui.add_sized([btn_width, btn_height], egui::Button::new(RichText::new("📁 文件夹").size(13.0).color(TEXT_PRIMARY))
                .fill(CARD_COLOR)
                .stroke(Stroke::new(1.0, BORDER_MEDIUM))
                .rounding(Rounding::same(10.0))).clicked() {
                self.select_folder();
            }
            ui.add_space(spacing);
            
            // 截图按钮 - 紫色，紧贴右边（外层已有16px边距）
            if ui.add_sized([btn_width, btn_height], egui::Button::new(RichText::new("📷 截图").size(13.0).color(Color32::WHITE))
                .fill(PURPLE_ACCENT)
                .rounding(Rounding::same(10.0))).clicked() {
                self.copy_screenshot();
            }
        });
    }
}
