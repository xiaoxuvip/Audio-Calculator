using System;
using System.IO;
using System.IO.Pipes;
using System.Threading;
using System.Threading.Tasks;
using System.Collections.Generic;

namespace AudioCalculator
{
    public partial class App : System.Windows.Application
    {
        private static Mutex? _mutex;
        private const string PipeName = "AudioCalculator_Pipe";
        private static MainWindow? _mainWindow;
        private static CancellationTokenSource? _cts;
        private static readonly List<string> _pendingFiles = new();
        private static readonly object _lock = new();
        private static System.Windows.Threading.DispatcherTimer? _loadTimer;
        private static DateTime _lastDataTime = DateTime.MinValue;
        private static int _expectedFileCount = 0;

        protected override void OnStartup(System.Windows.StartupEventArgs e)
        {
            base.OnStartup(e);

            try
            {
                _mutex = new Mutex(true, "AudioCalculator_SingleInstance", out bool isNew);

                if (!isNew)
                {
                    // 非首个实例：通过管道发送文件路径
                    if (e.Args.Length > 0)
                        SendFilesToMainInstance(e.Args);
                    _mutex = null;
                    Shutdown();
                    return;
                }

                // 首个实例：先启动管道服务器
                _cts = new CancellationTokenSource();
                Task.Run(() => StartPipeServer(_cts.Token));

                // 如果有命令行参数，添加到待处理列表
                if (e.Args.Length > 0)
                {
                    lock (_lock)
                    {
                        foreach (var arg in e.Args)
                        {
                            if (!string.IsNullOrWhiteSpace(arg))
                                _pendingFiles.Add(arg.Trim());
                        }
                        _lastDataTime = DateTime.Now;
                        _expectedFileCount = e.Args.Length;
                    }
                }

                // 创建主窗口
                _mainWindow = new MainWindow();
                _mainWindow.Show();

                // 使用定时器检查是否可以加载
                _loadTimer = new System.Windows.Threading.DispatcherTimer();
                _loadTimer.Interval = TimeSpan.FromMilliseconds(50);
                _loadTimer.Tick += LoadTimer_Tick;
                _loadTimer.Start();
            }
            catch
            {
                Shutdown();
            }
        }

        private static void LoadTimer_Tick(object? sender, EventArgs e)
        {
            lock (_lock)
            {
                if (_pendingFiles.Count == 0) return;
                
                // 根据文件数量动态调整等待时间
                // 文件越多，等待时间越长（每10个文件增加50ms，最多2秒）
                int waitTime = Math.Min(300 + (_pendingFiles.Count / 10) * 50, 2000);
                
                if ((DateTime.Now - _lastDataTime).TotalMilliseconds < waitTime) return;
            }
            
            LoadPendingFiles();
        }

        private static void LoadPendingFiles()
        {
            string[] files;
            lock (_lock)
            {
                if (_pendingFiles.Count == 0) return;
                files = _pendingFiles.ToArray();
                _pendingFiles.Clear();
            }
            
            _mainWindow?.AddFilesFromArgs(files);
            _mainWindow?.Activate();
        }

        private static void SendFilesToMainInstance(string[] args)
        {
            for (int retry = 0; retry < 10; retry++)
            {
                try
                {
                    using var client = new NamedPipeClientStream(".", PipeName, PipeDirection.Out);
                    client.Connect(1000);
                    
                    using var writer = new StreamWriter(client);
                    foreach (var arg in args)
                    {
                        if (!string.IsNullOrWhiteSpace(arg))
                            writer.WriteLine(arg.Trim());
                    }
                    writer.Flush();
                    return;
                }
                catch
                {
                    Thread.Sleep(100);
                }
            }
        }

        private static async Task StartPipeServer(CancellationToken token)
        {
            while (!token.IsCancellationRequested)
            {
                try
                {
                    using var server = new NamedPipeServerStream(PipeName, PipeDirection.In, 
                        NamedPipeServerStream.MaxAllowedServerInstances, 
                        PipeTransmissionMode.Byte, PipeOptions.Asynchronous);
                    
                    await server.WaitForConnectionAsync(token);
                    
                    using var reader = new StreamReader(server);
                    string? line;
                    while ((line = await reader.ReadLineAsync()) != null)
                    {
                        if (!string.IsNullOrWhiteSpace(line))
                        {
                            lock (_lock)
                            {
                                _pendingFiles.Add(line.Trim());
                                _lastDataTime = DateTime.Now;
                            }
                        }
                    }
                }
                catch (OperationCanceledException)
                {
                    break;
                }
                catch { }
            }
        }

        protected override void OnExit(System.Windows.ExitEventArgs e)
        {
            _loadTimer?.Stop();
            _cts?.Cancel();
            _cts?.Dispose();
            try { _mutex?.ReleaseMutex(); _mutex?.Dispose(); } catch { }
            base.OnExit(e);
        }
    }
}
