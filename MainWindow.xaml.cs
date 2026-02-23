using System;
using System.Collections.ObjectModel;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Windows.Controls;
using System.Windows.Media.Imaging;
using Microsoft.Win32;
using WinForms = System.Windows.Forms;
using WPF = System.Windows;
using Media = System.Windows.Media;

namespace AudioCalculator
{
    public partial class MainWindow : WPF.Window
    {
        private ObservableCollection<AudioFileInfo> _audioFiles = new();
        private TimeSpan _totalDuration = TimeSpan.Zero;
        public const string Version = "1.6";
        
        private static readonly string[] SupportedExtensions = 
        { 
            ".mp3", ".wav", ".flac", ".m4a", ".ogg", ".wma", ".aac", 
            ".aiff", ".ape", ".opus", ".webm", ".mp4", ".mkv" 
        };

        public MainWindow()
        {
            InitializeComponent();
            FileListView.ItemsSource = _audioFiles;
            UpdateUI();

            // 根据编译版本类型设置窗口标题
#if EDITION_FULL
            Title = $"音频时长统计与结算 完整版 v{Version} BY:Xiaoxu";
#elif EDITION_LITE
            Title = $"音频时长统计与结算 轻量版 v{Version} BY:Xiaoxu";
#else
            Title = $"音频时长统计与结算 v{Version} BY:Xiaoxu";
#endif

            // 恢复上次关闭时的窗口位置
            RestoreWindowPosition();
            
            // 窗口关闭时保存位置并清理资源
            this.Closed += (s, e) =>
            {
                SaveWindowPosition();
                _audioFiles.Clear();
                _totalDuration = TimeSpan.Zero;
            };
        }

        /// <summary>恢复窗口位置（从设置中读取上次关闭时的坐标）</summary>
        private void RestoreWindowPosition()
        {
            var settings = AppSettings.Instance;
            // 检查是否有保存的位置数据
            if (!double.IsNaN(settings.MainWindowLeft) && !double.IsNaN(settings.MainWindowTop))
            {
                // 验证位置是否在可见屏幕范围内
                var left = settings.MainWindowLeft;
                var top = settings.MainWindowTop;
                var screenWidth = WPF.SystemParameters.VirtualScreenWidth;
                var screenHeight = WPF.SystemParameters.VirtualScreenHeight;
                var screenLeft = WPF.SystemParameters.VirtualScreenLeft;
                var screenTop = WPF.SystemParameters.VirtualScreenTop;

                if (left >= screenLeft && left < screenLeft + screenWidth - 50 &&
                    top >= screenTop && top < screenTop + screenHeight - 50)
                {
                    WindowStartupLocation = WPF.WindowStartupLocation.Manual;
                    Left = left;
                    Top = top;
                }
            }
        }

        /// <summary>保存窗口位置到设置文件</summary>
        private void SaveWindowPosition()
        {
            var settings = AppSettings.Instance;
            settings.MainWindowLeft = Left;
            settings.MainWindowTop = Top;
            settings.Save();
        }

        private void OpenWebsite_Click(object sender, WPF.RoutedEventArgs e)
        {
            try
            {
                System.Diagnostics.Process.Start(new System.Diagnostics.ProcessStartInfo
                {
                    FileName = "https://github.com/xiaoxuvip/Audio-Calculator",
                    UseShellExecute = true
                });
            }
            catch { }
        }

        public void AddFilesFromArgs(string[] args)
        {
            var files = new System.Collections.Generic.List<string>();
            bool isFromContextMenu = args.Length > 0 && args.All(a => File.Exists(a));
            int originalArgCount = args.Length;
            
            foreach (var arg in args)
            {
                if (string.IsNullOrWhiteSpace(arg)) continue;
                
                try
                {
                    if (Directory.Exists(arg))
                    {
                        files.AddRange(Directory.GetFiles(arg, "*.*", SearchOption.AllDirectories)
                            .Where(f => SupportedExtensions.Contains(Path.GetExtension(f).ToLower())));
                    }
                    else if (File.Exists(arg))
                    {
                        files.Add(arg);
                    }
                }
                catch { }
            }
            
            // 只有当从右键菜单选择超过100个文件时才显示提示
            // Windows系统限制会导致只传递部分文件，此时originalArgCount会远小于用户实际选择的数量
            if (isFromContextMenu && originalArgCount >= 100 && _audioFiles.Count == 0)
            {
                WPF.MessageBox.Show(
                    "检测到可能选择了大量文件，但由于 Windows 系统限制，无法全部加载。\n\n" +
                    "💡 建议：请在音频文件所在的文件夹上右键，选择「音频时长统计」，\n" +
                    "软件将自动扫描文件夹内的所有音频文件，无数量限制。",
                    "提示", 
                    WPF.MessageBoxButton.OK, 
                    WPF.MessageBoxImage.Information);
            }
            
            AddFiles(files.ToArray());
        }

        private void AddContextMenu_Click(object sender, WPF.RoutedEventArgs e)
        {
            try
            {
                string exePath = System.Diagnostics.Process.GetCurrentProcess().MainModule?.FileName ?? "";
                if (string.IsNullOrEmpty(exePath)) return;

                // 图标路径格式：不带引号，带索引
                string iconPath = $"{exePath},0";

                // 文件右键菜单
                using (var key = Registry.ClassesRoot.CreateSubKey(@"*\shell\AudioCalculator"))
                {
                    key?.SetValue("", "音频时长统计");
                    key?.SetValue("Icon", iconPath);
                    key?.SetValue("MultiSelectModel", "Player");
                }
                using (var key = Registry.ClassesRoot.CreateSubKey(@"*\shell\AudioCalculator\command"))
                {
                    key?.SetValue("", $"\"{exePath}\" \"%1\"");
                }

                // 文件夹右键菜单
                using (var key = Registry.ClassesRoot.CreateSubKey(@"Directory\shell\AudioCalculator"))
                {
                    key?.SetValue("", "音频时长统计");
                    key?.SetValue("Icon", iconPath);
                }
                using (var key = Registry.ClassesRoot.CreateSubKey(@"Directory\shell\AudioCalculator\command"))
                {
                    key?.SetValue("", $"\"{exePath}\" \"%1\"");
                }

                try
                {
                    var psi = new System.Diagnostics.ProcessStartInfo
                    {
                        FileName = "ie4uinit.exe",
                        Arguments = "-show",
                        UseShellExecute = true,
                        CreateNoWindow = true
                    };
                    System.Diagnostics.Process.Start(psi);
                }
                catch { }

                WPF.MessageBox.Show("已添加到右键菜单！", "成功", WPF.MessageBoxButton.OK, WPF.MessageBoxImage.Information);
            }
            catch (UnauthorizedAccessException)
            {
                WPF.MessageBox.Show("需要管理员权限，请右键以管理员身份运行程序。", "权限不足", WPF.MessageBoxButton.OK, WPF.MessageBoxImage.Warning);
            }
            catch (Exception ex)
            {
                WPF.MessageBox.Show($"失败: {ex.Message}", "错误", WPF.MessageBoxButton.OK, WPF.MessageBoxImage.Error);
            }
        }

        private void RemoveContextMenu_Click(object sender, WPF.RoutedEventArgs e)
        {
            try
            {
                Registry.ClassesRoot.DeleteSubKeyTree(@"*\shell\AudioCalculator", false);
                Registry.ClassesRoot.DeleteSubKeyTree(@"Directory\shell\AudioCalculator", false);
                WPF.MessageBox.Show("已移除右键菜单！", "成功", WPF.MessageBoxButton.OK, WPF.MessageBoxImage.Information);
            }
            catch (UnauthorizedAccessException)
            {
                WPF.MessageBox.Show("需要管理员权限，请右键以管理员身份运行程序。", "权限不足", WPF.MessageBoxButton.OK, WPF.MessageBoxImage.Warning);
            }
            catch (Exception ex)
            {
                WPF.MessageBox.Show($"失败: {ex.Message}", "错误", WPF.MessageBoxButton.OK, WPF.MessageBoxImage.Error);
            }
        }

        /// <summary>打赏按钮 — 弹出打赏支持窗口</summary>
        private void Donate_Click(object sender, WPF.RoutedEventArgs e)
        {
            var donateWindow = new DonateWindow { Owner = this };
            donateWindow.ShowDialog();
        }

        private void SelectFiles_Click(object sender, WPF.RoutedEventArgs e)
        {
            var dialog = new Microsoft.Win32.OpenFileDialog
            {
                Multiselect = true,
                Filter = "音频文件|*.mp3;*.wav;*.flac;*.m4a;*.ogg;*.wma;*.aac;*.aiff;*.ape;*.opus|所有文件|*.*",
                Title = "选择音频文件"
            };

            if (dialog.ShowDialog() == true)
                AddFiles(dialog.FileNames);
        }

        private void SelectFolder_Click(object sender, WPF.RoutedEventArgs e)
        {
            var dialog = new WinForms.FolderBrowserDialog
            {
                Description = "选择包含音频文件的文件夹",
                ShowNewFolderButton = false
            };

            if (dialog.ShowDialog() == WinForms.DialogResult.OK)
            {
                var files = Directory.GetFiles(dialog.SelectedPath, "*.*", SearchOption.AllDirectories)
                    .Where(f => SupportedExtensions.Contains(Path.GetExtension(f).ToLower()))
                    .ToArray();
                AddFiles(files);
            }
        }

        private void AddFiles(string[] filePaths)
        {
            var newFiles = new System.Collections.Generic.List<AudioFileInfo>();
            
            foreach (var path in filePaths)
            {
                if (string.IsNullOrWhiteSpace(path) || !File.Exists(path)) continue;
                
                var ext = Path.GetExtension(path).ToLower();
                if (!SupportedExtensions.Contains(ext)) continue;
                if (_audioFiles.Any(f => f.FilePath == path)) continue;
                if (newFiles.Any(f => f.FilePath == path)) continue;

                try
                {
                    // 优先使用 Windows Shell API 获取时长（与资源管理器一致）
                    var duration = GetShellDuration(path);
                    
                    // 如果 Shell API 失败，回退到 TagLib
                    if (duration == TimeSpan.Zero)
                    {
                        using var file = TagLib.File.Create(path);
                        duration = file.Properties.Duration;
                    }
                    
                    newFiles.Add(new AudioFileInfo
                    {
                        FilePath = path,
                        FileName = Path.GetFileName(path),
                        Duration = duration,
                        DurationText = FormatDuration(duration)
                    });
                }
                catch { }
            }

            // 按文件名排序后添加
            foreach (var f in newFiles.OrderBy(f => f.FileName, StringComparer.OrdinalIgnoreCase))
                _audioFiles.Add(f);

            // 重新排序整个列表
            var sorted = _audioFiles.OrderBy(f => f.FileName, StringComparer.OrdinalIgnoreCase).ToList();
            _audioFiles.Clear();
            foreach (var f in sorted)
                _audioFiles.Add(f);

            UpdateUI();
        }

        private void ClearFiles_Click(object sender, WPF.RoutedEventArgs e)
        {
            _audioFiles.Clear();
            UpdateUI();
        }

        private void FileListView_DragOver(object sender, WPF.DragEventArgs e)
        {
            e.Effects = e.Data.GetDataPresent(WPF.DataFormats.FileDrop) 
                ? WPF.DragDropEffects.Copy : WPF.DragDropEffects.None;
            e.Handled = true;
        }

        private void FileListView_Drop(object sender, WPF.DragEventArgs e)
        {
            if (!e.Data.GetDataPresent(WPF.DataFormats.FileDrop)) return;
            var files = (string[])e.Data.GetData(WPF.DataFormats.FileDrop);
            AddFilesFromArgs(files);
        }

        private void PriceInput_TextChanged(object sender, TextChangedEventArgs e) => CalculatePrice();
        private void Unit_Changed(object sender, WPF.RoutedEventArgs e) => CalculatePrice();

        private void UpdateUI()
        {
            _totalDuration = TimeSpan.Zero;
            foreach (var file in _audioFiles)
                _totalDuration += file.Duration;

            FileCountText.Text = $"({_audioFiles.Count} 个)";
            TotalDurationText.Text = FormatDuration(_totalDuration);
            EmptyState.Visibility = _audioFiles.Count == 0 ? WPF.Visibility.Visible : WPF.Visibility.Collapsed;
            CalculatePrice();
        }

        private void CalculatePrice()
        {
            if (!double.TryParse(PriceInput?.Text, out double price)) price = 0;
            double totalPrice = RadioMinute?.IsChecked == true 
                ? _totalDuration.TotalMinutes * price 
                : _totalDuration.TotalHours * price;
            if (TotalPriceText != null)
                TotalPriceText.Text = $"¥{totalPrice:F2}";
        }

        private void CopyScreenshot_Click(object sender, WPF.RoutedEventArgs e)
        {
            try
            {
                var screenshot = GenerateScreenshot();
                WPF.Clipboard.SetImage(screenshot);
                WPF.MessageBox.Show("截图已复制到剪贴板！", "成功", WPF.MessageBoxButton.OK, WPF.MessageBoxImage.Information);
            }
            catch (Exception ex)
            {
                WPF.MessageBox.Show($"截图失败: {ex.Message}", "错误", WPF.MessageBoxButton.OK, WPF.MessageBoxImage.Error);
            }
        }

        private BitmapSource GenerateScreenshot()
        {
            return AppSettings.Instance.ScreenshotStyle switch
            {
                "Gradient" => GenerateGradientScreenshot(),
                "Dark" => GenerateDarkScreenshot(),
                _ => GenerateSimpleScreenshot()
            };
        }

        private BitmapSource GenerateSimpleScreenshot()
        {
            int width = 760;
            var font = CreateTypeface(WPF.FontWeights.Normal);
            var boldFont = CreateTypeface(WPF.FontWeights.Bold);
            
            // 计算文件列表所需高度
            int fileListHeight = CalculateFileListHeight(_audioFiles.Count, 28);
            int height = 320 + fileListHeight + 80;
            
            var visual = new Media.DrawingVisual();
            
            using (var ctx = visual.RenderOpen())
            {
                ctx.DrawRectangle(Media.Brushes.White, null, new WPF.Rect(0, 0, width, height));
                
                DrawText(ctx, "音频时长统计结算单", boldFont, 36, Media.Brushes.Black, 48, 48);
                ctx.DrawLine(new Media.Pen(Media.Brushes.LightGray, 2), new WPF.Point(48, 104), new WPF.Point(width - 48, 104));
                
                double y = 136;
                DrawText(ctx, $"📁 文件数量: {_audioFiles.Count} 个", font, 24, Media.Brushes.DimGray, 48, y); y += 40;
                DrawText(ctx, $"⏱️ 总时长: {FormatDuration(_totalDuration)}", font, 24, Media.Brushes.DimGray, 48, y); y += 40;
                
                var unit = RadioMinute?.IsChecked == true ? "分钟" : "小时";
                DrawText(ctx, $"💵 单价: ¥{PriceInput.Text}/{unit}", font, 24, Media.Brushes.DimGray, 48, y); y += 50;
                
                // 绘制文件列表
                if (_audioFiles.Count > 0)
                {
                    DrawText(ctx, "📋 文件列表:", font, 20, Media.Brushes.Gray, 48, y); y += 32;
                    y = DrawFileList(ctx, font, 18, Media.Brushes.DimGray, 48, y, width - 96);
                    y += 20;
                }
                
                ctx.DrawLine(new Media.Pen(Media.Brushes.LightGray, 2), new WPF.Point(48, y), new WPF.Point(width - 48, y));
                y += 28;
                
                DrawText(ctx, $"💰 总费用: {TotalPriceText.Text}", boldFont, 36, 
                    new Media.SolidColorBrush(Media.Color.FromRgb(16, 185, 129)), 48, y);
                
                DrawText(ctx, $"生成时间: {DateTime.Now:yyyy-MM-dd HH:mm}", font, 18, Media.Brushes.LightGray, 48, height - 40);
            }

            var bitmap = new RenderTargetBitmap(width, height, 96, 96, Media.PixelFormats.Pbgra32);
            bitmap.Render(visual);
            return bitmap;
        }

        private BitmapSource GenerateGradientScreenshot()
        {
            int width = 760;
            var font = CreateTypeface(WPF.FontWeights.Normal);
            var boldFont = CreateTypeface(WPF.FontWeights.Bold);
            
            // 计算文件列表所需高度
            int fileListHeight = CalculateFileListHeight(_audioFiles.Count, 26);
            int height = 340 + fileListHeight + 80;
            
            var visual = new Media.DrawingVisual();
            
            using (var ctx = visual.RenderOpen())
            {
                var gradient = new Media.LinearGradientBrush(
                    Media.Color.FromRgb(99, 102, 241),
                    Media.Color.FromRgb(236, 72, 153), 45);
                ctx.DrawRoundedRectangle(gradient, null, new WPF.Rect(0, 0, width, height), 32, 32);
                
                var cardBrush = new Media.SolidColorBrush(Media.Color.FromArgb(245, 255, 255, 255));
                ctx.DrawRoundedRectangle(cardBrush, null, new WPF.Rect(32, 32, width - 64, height - 64), 24, 24);
                
                // 标题
                DrawText(ctx, "🎵 音频时长统计", boldFont, 28, 
                    new Media.SolidColorBrush(Media.Color.FromRgb(99, 102, 241)), 64, 64);
                
                double y = 112;
                
                // 三个信息放在一行，带底色
                var infoBg1 = new Media.SolidColorBrush(Media.Color.FromRgb(238, 242, 255));
                var infoBg2 = new Media.SolidColorBrush(Media.Color.FromRgb(254, 243, 199));
                var infoBg3 = new Media.SolidColorBrush(Media.Color.FromRgb(220, 252, 231));
                
                ctx.DrawRoundedRectangle(infoBg1, null, new WPF.Rect(64, y, 200, 36), 8, 8);
                ctx.DrawRoundedRectangle(infoBg2, null, new WPF.Rect(274, y, 200, 36), 8, 8);
                ctx.DrawRoundedRectangle(infoBg3, null, new WPF.Rect(484, y, 200, 36), 8, 8);
                
                var unit = RadioMinute?.IsChecked == true ? "分钟" : "小时";
                DrawText(ctx, $"📁 {_audioFiles.Count} 个文件", font, 18, Media.Brushes.DimGray, 76, y + 8);
                DrawText(ctx, $"⏱️ {FormatDuration(_totalDuration)}", font, 18, Media.Brushes.DimGray, 286, y + 8);
                DrawText(ctx, $"💵 ¥{PriceInput.Text}/{unit}", font, 18, Media.Brushes.DimGray, 496, y + 8);
                
                y += 52;
                
                // 绘制文件列表
                if (_audioFiles.Count > 0)
                {
                    DrawText(ctx, "文件列表:", font, 18, Media.Brushes.Gray, 64, y); y += 28;
                    y = DrawFileList(ctx, font, 16, Media.Brushes.DimGray, 64, y, width - 128);
                    y += 16;
                }
                
                DrawText(ctx, "总计金额", font, 20, Media.Brushes.Gray, 64, y); y += 32;
                DrawText(ctx, TotalPriceText.Text, boldFont, 44, 
                    new Media.SolidColorBrush(Media.Color.FromRgb(16, 185, 129)), 64, y);
                
                DrawText(ctx, DateTime.Now.ToString("yyyy-MM-dd HH:mm"), font, 16, Media.Brushes.LightGray, 64, height - 72);
            }

            var bitmap = new RenderTargetBitmap(width, height, 96, 96, Media.PixelFormats.Pbgra32);
            bitmap.Render(visual);
            return bitmap;
        }

        private BitmapSource GenerateDarkScreenshot()
        {
            int width = 760;
            var font = CreateTypeface(WPF.FontWeights.Normal);
            var boldFont = CreateTypeface(WPF.FontWeights.Bold);
            var lightText = new Media.SolidColorBrush(Media.Color.FromRgb(148, 163, 184));
            var dimText = new Media.SolidColorBrush(Media.Color.FromRgb(100, 116, 139));
            
            // 计算文件列表所需高度
            int fileListHeight = CalculateFileListHeight(_audioFiles.Count, 28);
            int height = 320 + fileListHeight + 80;
            
            var visual = new Media.DrawingVisual();
            
            using (var ctx = visual.RenderOpen())
            {
                var bgBrush = new Media.SolidColorBrush(Media.Color.FromRgb(30, 41, 59));
                ctx.DrawRoundedRectangle(bgBrush, null, new WPF.Rect(0, 0, width, height), 24, 24);
                
                DrawText(ctx, "音频时长统计结算单", boldFont, 32, Media.Brushes.White, 48, 48);
                
                ctx.DrawLine(new Media.Pen(new Media.SolidColorBrush(Media.Color.FromRgb(51, 65, 85)), 2), 
                    new WPF.Point(48, 100), new WPF.Point(width - 48, 100));
                
                double y = 124;
                DrawText(ctx, $"📁 文件数量: {_audioFiles.Count} 个", font, 22, lightText, 48, y); y += 36;
                DrawText(ctx, $"⏱️ 总时长: {FormatDuration(_totalDuration)}", font, 22, lightText, 48, y); y += 36;
                
                var unit = RadioMinute?.IsChecked == true ? "分钟" : "小时";
                DrawText(ctx, $"💵 单价: ¥{PriceInput.Text}/{unit}", font, 22, lightText, 48, y); y += 44;
                
                // 绘制文件列表
                if (_audioFiles.Count > 0)
                {
                    DrawText(ctx, "📋 文件列表:", font, 18, dimText, 48, y); y += 28;
                    y = DrawFileList(ctx, font, 16, lightText, 48, y, width - 96);
                    y += 16;
                }
                
                ctx.DrawLine(new Media.Pen(new Media.SolidColorBrush(Media.Color.FromRgb(51, 65, 85)), 2), 
                    new WPF.Point(48, y), new WPF.Point(width - 48, y));
                y += 24;
                
                DrawText(ctx, $"💰 总费用: {TotalPriceText.Text}", boldFont, 32, 
                    new Media.SolidColorBrush(Media.Color.FromRgb(52, 211, 153)), 48, y);
                
                DrawText(ctx, $"生成时间: {DateTime.Now:yyyy-MM-dd HH:mm}", font, 16, 
                    new Media.SolidColorBrush(Media.Color.FromRgb(71, 85, 105)), 48, height - 36);
            }

            var bitmap = new RenderTargetBitmap(width, height, 96, 96, Media.PixelFormats.Pbgra32);
            bitmap.Render(visual);
            return bitmap;
        }

        private Media.Typeface CreateTypeface(WPF.FontWeight weight)
        {
            return new Media.Typeface(new Media.FontFamily("Microsoft YaHei UI"), WPF.FontStyles.Normal, weight, WPF.FontStretches.Normal);
        }

        private void DrawText(Media.DrawingContext ctx, string text, Media.Typeface typeface, double size, Media.Brush brush, double x, double y)
        {
            var formattedText = new Media.FormattedText(text, System.Globalization.CultureInfo.CurrentCulture,
                WPF.FlowDirection.LeftToRight, typeface, size, brush, 96);
            ctx.DrawText(formattedText, new WPF.Point(x, y));
        }

        private string FormatDuration(TimeSpan duration)
        {
            return $"{(int)duration.TotalHours:D2}:{duration.Minutes:D2}:{duration.Seconds:D2}";
        }

        /// <summary>
        /// 使用 Windows Shell API 获取音频时长（与资源管理器显示一致）
        /// </summary>
        private TimeSpan GetShellDuration(string path)
        {
            return ShellPropertyReader.GetDuration(path);
        }

        // 计算文件列表所需高度（支持2列布局）
        private int CalculateFileListHeight(int fileCount, int lineHeight)
        {
            if (fileCount == 0) return 0;
            
            int cols = fileCount > 12 ? 2 : 1; // 超过12个文件用2列
            int rows = (int)Math.Ceiling((double)fileCount / cols);
            
            return rows * lineHeight + 8;
        }

        // 绘制文件列表（支持2列布局，单列时居中显示）
        private double DrawFileList(Media.DrawingContext ctx, Media.Typeface typeface, double fontSize, 
            Media.Brush brush, double x, double y, double totalWidth)
        {
            if (_audioFiles.Count == 0) return y;
            
            int cols = _audioFiles.Count > 12 ? 2 : 1;
            double lineHeight = fontSize + 10;
            
            if (cols == 1)
            {
                // 单列：逐行显示
                for (int i = 0; i < _audioFiles.Count; i++)
                {
                    var file = _audioFiles[i];
                    string displayName = file.FileName;
                    
                    // 截断过长的文件名
                    if (displayName.Length > 40)
                        displayName = displayName.Substring(0, 37) + "...";
                    
                    string text = $"• {displayName} ({file.DurationText})";
                    DrawText(ctx, text, typeface, fontSize, brush, x, y + i * lineHeight);
                }
                return y + _audioFiles.Count * lineHeight;
            }
            else
            {
                // 双列布局
                double colWidth = totalWidth / 2 - 8;
                int itemsPerCol = (int)Math.Ceiling((double)_audioFiles.Count / 2);
                
                for (int i = 0; i < _audioFiles.Count; i++)
                {
                    int col = i / itemsPerCol;
                    int row = i % itemsPerCol;
                    
                    double itemX = x + col * (colWidth + 16);
                    double itemY = y + row * lineHeight;
                    
                    var file = _audioFiles[i];
                    string displayName = file.FileName;
                    
                    // 截断过长的文件名（双列时更短）
                    if (displayName.Length > 20)
                        displayName = displayName.Substring(0, 17) + "...";
                    
                    string text = $"• {displayName} ({file.DurationText})";
                    DrawText(ctx, text, typeface, fontSize, brush, itemX, itemY);
                }
                
                int rows = (int)Math.Ceiling((double)_audioFiles.Count / 2);
                return y + rows * lineHeight;
            }
        }
    }

    public class AudioFileInfo
    {
        public string FilePath { get; set; } = "";
        public string FileName { get; set; } = "";
        public TimeSpan Duration { get; set; }
        public string DurationText { get; set; } = "";
    }

    /// <summary>
    /// Windows Shell 属性读取器，用于获取与资源管理器一致的媒体时长
    /// </summary>
    public static class ShellPropertyReader
    {
        // Shell32 COM 接口
        [DllImport("shell32.dll", CharSet = CharSet.Unicode)]
        private static extern int SHGetPropertyStoreFromParsingName(
            string pszPath,
            IntPtr pbc,
            GETPROPERTYSTOREFLAGS flags,
            ref Guid riid,
            out IPropertyStore ppv);

        [Flags]
        private enum GETPROPERTYSTOREFLAGS : uint
        {
            GPS_DEFAULT = 0,
            GPS_HANDLERPROPERTIESONLY = 0x1,
            GPS_READWRITE = 0x2,
            GPS_TEMPORARY = 0x4,
            GPS_FASTPROPERTIESONLY = 0x8,
            GPS_OPENSLOWITEM = 0x10,
            GPS_DELAYCREATION = 0x20,
            GPS_BESTEFFORT = 0x40,
            GPS_NO_OPLOCK = 0x80,
            GPS_MASK_VALID = 0xff
        }

        [ComImport]
        [Guid("886D8EEB-8CF2-4446-8D02-CDBA1DBDCF99")]
        [InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
        private interface IPropertyStore
        {
            int GetCount(out uint cProps);
            int GetAt(uint iProp, out PROPERTYKEY pkey);
            int GetValue(ref PROPERTYKEY key, out PROPVARIANT pv);
            int SetValue(ref PROPERTYKEY key, ref PROPVARIANT pv);
            int Commit();
        }

        [StructLayout(LayoutKind.Sequential, Pack = 4)]
        private struct PROPERTYKEY
        {
            public Guid fmtid;
            public uint pid;
        }

        [StructLayout(LayoutKind.Explicit)]
        private struct PROPVARIANT
        {
            [FieldOffset(0)] public ushort vt;
            [FieldOffset(8)] public long hVal;  // For VT_I8 / VT_UI8
            [FieldOffset(8)] public IntPtr ptr;

            public void Clear()
            {
                PropVariantClear(ref this);
            }
        }

        [DllImport("ole32.dll")]
        private static extern int PropVariantClear(ref PROPVARIANT pvar);

        // System.Media.Duration 属性键
        private static readonly PROPERTYKEY PKEY_Media_Duration = new PROPERTYKEY
        {
            fmtid = new Guid("64440490-4C8B-11D1-8B70-080036B11A03"),
            pid = 3
        };

        private static readonly Guid IID_IPropertyStore = new Guid("886D8EEB-8CF2-4446-8D02-CDBA1DBDCF99");

        public static TimeSpan GetDuration(string filePath)
        {
            if (string.IsNullOrEmpty(filePath) || !File.Exists(filePath))
                return TimeSpan.Zero;

            IPropertyStore? store = null;
            try
            {
                var iid = IID_IPropertyStore;
                int hr = SHGetPropertyStoreFromParsingName(filePath, IntPtr.Zero, 
                    GETPROPERTYSTOREFLAGS.GPS_DEFAULT, ref iid, out store);
                
                if (hr != 0 || store == null)
                    return TimeSpan.Zero;

                var key = PKEY_Media_Duration;
                hr = store.GetValue(ref key, out PROPVARIANT pv);
                
                if (hr != 0)
                    return TimeSpan.Zero;

                try
                {
                    // 时长以 100 纳秒为单位存储
                    if (pv.vt == 21) // VT_UI8
                    {
                        return TimeSpan.FromTicks(pv.hVal);
                    }
                }
                finally
                {
                    pv.Clear();
                }
            }
            catch
            {
                // 忽略错误
            }
            finally
            {
                if (store != null)
                    Marshal.ReleaseComObject(store);
            }

            return TimeSpan.Zero;
        }
    }
}
