# 🎵 音频时长统计与结算工具

一款 Windows 桌面应用，用于快速统计音频文件时长并计算费用。

## 功能特性

- ✅ 支持多种音频格式：MP3, WAV, FLAC, M4A, OGG, WMA, AAC, AIFF, APE, OPUS
- ✅ 拖拽文件/文件夹快速添加
- ✅ 自动计算总时长
- ✅ 灵活的计费设置（按小时/分钟）
- ✅ 一键生成截图到剪贴板
- ✅ 支持 Windows 右键菜单集成

## 下载使用

前往 [Releases](https://github.com/xiaoxuvip/Audio-Calculator/releases) 页面下载，提供两个版本：

| 文件名 | 说明 | 体积 | 是否需要 .NET 运行时 |
|--------|------|------|---------------------|
| `AudioCalculator_v1.6.0_lite.exe` | 轻量版 | 较小 | 需要安装 [.NET 8 运行时](https://dotnet.microsoft.com/download/dotnet/8.0) |
| `AudioCalculator_v1.6.0_full.exe` | 完整版 | 较大 | 无需安装，开箱即用 |

> 不确定选哪个？推荐下载完整版（full），双击即可运行，无需额外安装任何依赖。

## 使用方法

### 方式一：直接运行
1. 双击 `AudioCalculator.exe` 启动程序
2. 点击"选择文件"或"选择文件夹"添加音频
3. 或直接拖拽文件到窗口中
4. 设置单价，自动计算费用
5. 点击"复制截图"分享结果

### 方式二：右键菜单
1. 双击 `RegisterContextMenu.reg` 注册右键菜单
2. 选中音频文件 → 右键 → "音频时长统计"
3. 自动打开程序并加载选中的文件

## 编译方法

需要 [.NET 8 SDK](https://dotnet.microsoft.com/download/dotnet/8.0)

```bash
# 编译轻量版（需要目标机器安装 .NET 8 运行时）
dotnet publish -c Release -r win-x64 --self-contained false -p:PublishSingleFile=true -o publish-lite

# 编译完整版（内置运行时，开箱即用）
dotnet publish -c Release -r win-x64 --self-contained true -p:PublishSingleFile=true -o publish-full
```

## 注册/移除右键菜单

- 注册：双击 `RegisterContextMenu.reg`
- 移除：双击 `UnregisterContextMenu.reg`

注意：需要修改 .reg 文件中的路径为实际安装路径。

## 技术栈

- .NET 8 + WPF
- TagLib-Sharp（音频元数据读取）

## 许可证

MIT License
