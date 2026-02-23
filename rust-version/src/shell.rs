//! Windows 右键菜单注册

#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;

/// 添加右键菜单
pub fn add_context_menu() -> Result<(), String> {
    #[cfg(windows)]
    {
        let exe_path = std::env::current_exe()
            .map_err(|e| format!("获取程序路径失败: {}", e))?;
        let exe_path_str = exe_path.to_string_lossy();
        let icon_path = format!("{},0", exe_path_str);
        
        // 文件右键菜单
        let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
        
        let (key, _) = hkcr.create_subkey(r"*\shell\AudioCalculator")
            .map_err(|e| format!("创建注册表项失败: {}。需要管理员权限，请右键以管理员身份运行程序。", e))?;
        key.set_value("", &"音频时长统计")
            .map_err(|e| format!("设置注册表值失败: {}", e))?;
        key.set_value("Icon", &icon_path)
            .map_err(|e| format!("设置图标失败: {}", e))?;
        key.set_value("MultiSelectModel", &"Player")
            .map_err(|e| format!("设置多选模式失败: {}", e))?;
        
        let (cmd_key, _) = hkcr.create_subkey(r"*\shell\AudioCalculator\command")
            .map_err(|e| format!("创建命令项失败: {}", e))?;
        cmd_key.set_value("", &format!("\"{}\" \"%1\"", exe_path_str))
            .map_err(|e| format!("设置命令失败: {}", e))?;
        
        // 文件夹右键菜单
        let (dir_key, _) = hkcr.create_subkey(r"Directory\shell\AudioCalculator")
            .map_err(|e| format!("创建文件夹菜单项失败: {}", e))?;
        dir_key.set_value("", &"音频时长统计")
            .map_err(|e| format!("设置文件夹菜单名称失败: {}", e))?;
        dir_key.set_value("Icon", &icon_path)
            .map_err(|e| format!("设置文件夹菜单图标失败: {}", e))?;
        
        let (dir_cmd_key, _) = hkcr.create_subkey(r"Directory\shell\AudioCalculator\command")
            .map_err(|e| format!("创建文件夹命令项失败: {}", e))?;
        dir_cmd_key.set_value("", &format!("\"{}\" \"%1\"", exe_path_str))
            .map_err(|e| format!("设置文件夹命令失败: {}", e))?;
        
        // 刷新图标缓存
        let _ = std::process::Command::new("ie4uinit.exe")
            .arg("-show")
            .spawn();
        
        Ok(())
    }
    
    #[cfg(not(windows))]
    {
        Err("右键菜单功能仅支持 Windows 系统".to_string())
    }
}

/// 移除右键菜单
pub fn remove_context_menu() -> Result<(), String> {
    #[cfg(windows)]
    {
        let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
        
        let _ = hkcr.delete_subkey_all(r"*\shell\AudioCalculator");
        let _ = hkcr.delete_subkey_all(r"Directory\shell\AudioCalculator");
        
        Ok(())
    }
    
    #[cfg(not(windows))]
    {
        Err("右键菜单功能仅支持 Windows 系统".to_string())
    }
}
