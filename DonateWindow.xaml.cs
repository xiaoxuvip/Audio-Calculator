// 打赏支持窗口 Code-Behind
// 显示微信收款码和获取最新版链接

using System.Windows;
using System.Windows.Navigation;

namespace AudioCalculator
{
    /// <summary>
    /// 打赏支持窗口交互逻辑
    /// </summary>
    public partial class DonateWindow : Window
    {
        public DonateWindow()
        {
            InitializeComponent();
        }

        /// <summary>超链接导航 — 用默认浏览器打开</summary>
        private void Hyperlink_Navigate(object sender, RequestNavigateEventArgs e)
        {
            try
            {
                System.Diagnostics.Process.Start(new System.Diagnostics.ProcessStartInfo
                {
                    FileName = e.Uri.AbsoluteUri,
                    UseShellExecute = true
                });
            }
            catch { }
            e.Handled = true;
        }

        /// <summary>文字点击打开网站（备用）</summary>
        private void Website_Click(object sender, System.Windows.Input.MouseButtonEventArgs e)
        {
            try
            {
                System.Diagnostics.Process.Start(new System.Diagnostics.ProcessStartInfo
                {
                    FileName = "https://www.xiaoxu.vip",
                    UseShellExecute = true
                });
            }
            catch { }
        }
    }
}
