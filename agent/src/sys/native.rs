//! Platform-native system information

pub fn get_hostname() -> String {
    #[cfg(windows)]
    {
        // TODO: Win32 GetComputerNameW
        "WORKSTATION".to_string()
    }

    #[cfg(not(windows))]
    {
        std::env::var("HOSTNAME")
            .or_else(|_| std::process::Command::new("hostname")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()))
            .unwrap_or_else(|_| "unknown".to_string())
    }
}

pub fn get_username() -> String {
    #[cfg(windows)]
    {
        // TODO: Win32 GetUserNameW
        "SYSTEM".to_string()
    }

    #[cfg(not(windows))]
    {
        std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
    }
}

pub fn get_pid() -> u32 {
    std::process::id()
}

pub fn get_os() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        std::env::consts::OS
    }
}

pub fn get_arch() -> &'static str {
    if cfg!(target_arch = "x86_64") {
        "x64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        "x86"
    }
}
