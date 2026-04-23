//! Platform-native system information

pub fn get_hostname() -> String {
    #[cfg(windows)]
    {
        std::env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".to_string())
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
        std::env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string())
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

pub fn get_internal_ip() -> Option<String> {
    #[cfg(windows)]
    {
        // Connect a UDP socket to a public address to determine the local interface IP.
        // No traffic is actually sent.
        use std::net::UdpSocket;
        let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
        socket.connect("8.8.8.8:80").ok()?;
        Some(socket.local_addr().ok()?.ip().to_string())
    }

    #[cfg(not(windows))]
    {
        use std::net::UdpSocket;
        let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
        socket.connect("8.8.8.8:80").ok()?;
        Some(socket.local_addr().ok()?.ip().to_string())
    }
}

pub fn get_privilege_info() -> String {
    #[cfg(windows)]
    {
        let username = std::env::var("USERNAME").unwrap_or_default();
        if username == "SYSTEM" || username == "LOCAL SERVICE" || username == "NETWORK SERVICE" {
            return "SYSTEM".to_string();
        }
        // Try to read SAM to check for admin elevation
        let is_admin = std::fs::read(r"C:\Windows\System32\config\SAM").is_ok();
        // Get enabled privileges via whoami
        if let Ok(output) = std::process::Command::new("whoami")
            .args(["/priv"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let enabled: Vec<&str> = stdout
                .lines()
                .filter(|l| l.contains("Enabled"))
                .filter_map(|l| l.split_whitespace().next())
                .filter(|name| *name != "SeChangeNotifyPrivilege") // skip trivial privilege
                .collect();
            if !enabled.is_empty() {
                let prefix = if is_admin { "Admin" } else { "User" };
                return format!("{}: {}", prefix, enabled.join(", "));
            }
        }
        if is_admin {
            "Admin".to_string()
        } else {
            "User".to_string()
        }
    }

    #[cfg(not(windows))]
    {
        let uid = std::process::Command::new("id")
            .arg("-u")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "?".to_string());
        if uid == "0" {
            "root".to_string()
        } else {
            format!("user (uid={})", uid)
        }
    }
}
