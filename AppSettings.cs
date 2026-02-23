using System;
using System.IO;
using System.Text.Json;

namespace AudioCalculator
{
    public class AppSettings
    {
        public string ScreenshotStyle { get; set; } = "Gradient"; // 默认渐变样式

        // 主窗口位置记忆
        public double MainWindowLeft { get; set; } = double.NaN;
        public double MainWindowTop { get; set; } = double.NaN;
        public double MainWindowWidth { get; set; } = double.NaN;
        public double MainWindowHeight { get; set; } = double.NaN;

        private static readonly string SettingsPath = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData),
            "AudioCalculator", "settings.json");

        private static AppSettings? _instance;
        public static AppSettings Instance => _instance ??= Load();

        public static AppSettings Load()
        {
            try
            {
                if (File.Exists(SettingsPath))
                {
                    var json = File.ReadAllText(SettingsPath);
                    return JsonSerializer.Deserialize<AppSettings>(json) ?? new AppSettings();
                }
            }
            catch { }
            return new AppSettings();
        }

        public void Save()
        {
            try
            {
                var dir = Path.GetDirectoryName(SettingsPath);
                if (!string.IsNullOrEmpty(dir) && !Directory.Exists(dir))
                    Directory.CreateDirectory(dir);

                var json = JsonSerializer.Serialize(this, new JsonSerializerOptions { WriteIndented = true });
                File.WriteAllText(SettingsPath, json);
            }
            catch { }
        }
    }
}
