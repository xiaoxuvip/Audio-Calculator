using System;
using Microsoft.Win32;
using WPF = System.Windows;

namespace AudioCalculator
{
    public partial class SettingsWindow : WPF.Window
    {
        public const string Version = "0.6";

        public SettingsWindow()
        {
            InitializeComponent();
            VersionText.Text = Version;
        }

        private void AddContextMenu_Click(object sender, WPF.RoutedEventArgs e)
        {
            try
            {
                string exePath = System.Diagnostics.Process.GetCurrentProcess().MainModule?.FileName ?? "";
                if (string.IsNullOrEmpty(exePath)) return;

                // 图标路径格式：exe路径,图标索引（0表示第一个图标）
                string iconPath = $"\"{exePath}\",0";

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

                using (var key = Registry.ClassesRoot.CreateSubKey(@"Directory\shell\AudioCalculator"))
                {
                    key?.SetValue("", "音频时长统计");
                    key?.SetValue("Icon", iconPath);
                }
                using (var key = Registry.ClassesRoot.CreateSubKey(@"Directory\shell\AudioCalculator\command"))
                {
                    key?.SetValue("", $"\"{exePath}\" \"%1\"");
                }

                // 刷新Windows图标缓存
                RefreshIconCache();

                WPF.MessageBox.Show("已添加到右键菜单！\n\n如果图标未更新，请重启资源管理器或注销重新登录。", "成功", WPF.MessageBoxButton.OK, WPF.MessageBoxImage.Information);
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

        private void RefreshIconCache()
        {
            try
            {
                // 通知Shell刷新图标
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

        private void Close_Click(object sender, WPF.RoutedEventArgs e)
        {
            Close();
        }
    }
}
