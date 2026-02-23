# 🎵 音频时长统计与结算工具 - Rust 版

这是 AudioCalculator 的 Rust 重写版本，功能与 .NET 版本完全一致。

## 功能特性

- ✅ 支持多种音频格式：MP3, WAV, FLAC, M4A, OGG, WMA, AAC, AIFF, APE, OPUS, WEBM, MP4, MKV
- ✅ 拖拽文件/文件夹快速添加
- ✅ 自动计算总时长
- ✅ 灵活的计费设置（按小时/分钟）
- ✅ 一键生成截图到剪贴板
- ✅ 支持 Windows 右键菜单集成
- ✅ 使用 Windows Shell API 获取时长（与资源管理器显示一致）

## 技术栈

- Rust 2021 Edition
- egui/eframe - 跨平台 GUI 框架
- symphonia - 音频解码库
- windows-rs - Windows API 绑定

## 编译方法

需要安装 Rust 工具链：https://rustup.rs/

```bash
# 开发模式编译
cargo build

# 发布模式编译（优化）
cargo build --release
```

编译产物位于 `target/release/audio-calculator.exe`

## 与 .NET 版本的区别

| 特性 | .NET 版本 | Rust 版本 |
|------|-----------|-----------|
| 运行时依赖 | 需要 .NET 7 运行时 | 无依赖，独立运行 |
| 可执行文件大小 | ~150MB (self-contained) | ~10MB |
| 启动速度 | 较慢 | 快 |
| GUI 框架 | WPF | egui |
| 截图功能 | 完整文字渲染 | 简化版（色块） |

## 许可证

MIT License
