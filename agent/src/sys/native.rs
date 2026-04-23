//! Platform-native system information

pub fn get_hostname() -> String {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::SystemInformation::{GetComputerNameExW, ComputerNamePhysicalDnsHostname};
        let mut buf = [0u16; 256];
        let mut size = buf.len() as u32;
        if unsafe { GetComputerNameExW(ComputerNamePhysicalDnsHostname, buf.as_mut_ptr(), &mut size) } != 0 {
            String::from_utf16_lossy(&buf[..size as usize])
        } else {
            "unknown".to_string()
        }
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
        use windows_sys::Win32::System::SystemInformation::{GetUserNameW};
        let mut buf = [0u16; 256];
        let mut size = buf.len() as u32;
        if unsafe { GetUserNameW(buf.as_mut_ptr(), &mut size) } != 0 {
            String::from_utf16_lossy(&buf[..size as usize - 1]) // size includes null terminator
        } else {
            "unknown".to_string()
        }
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

pub fn is_elevated() -> bool {
    #[cfg(windows)]
    {
        // On Windows, check if we can open a privileged process token
        use windows_sys::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
        use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
        use windows_sys::Win32::Foundation::HANDLE;
        let mut token: HANDLE = std::ptr::null_mut();
        if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) } == 0 {
            return false;
        }
        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut size = 0u32;
        let result = unsafe {
            GetTokenInformation(
                token,
                TokenElevation,
                &mut elevation as *mut _ as *mut _,
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut size,
            )
        };
        unsafe { windows_sys::Win32::Foundation::CloseHandle(token) };
        result != 0 && elevation.TokenIsElevated != 0
    }

    #[cfg(not(windows))]
    {
        // Unix: check if uid == 0
        std::process::Command::new("id")
            .arg("-u")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
            .unwrap_or(false)
    }
}
