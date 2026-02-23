fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../logo.ico");
        res.set("ProductName", "音频时长统计");
        res.set("FileDescription", "音频时长统计与结算工具");
        res.set("LegalCopyright", "Copyright © 2024 晓旭");
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resources: {}", e);
        }
    }
}
