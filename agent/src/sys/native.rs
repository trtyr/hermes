//! Platform-native system information

pub fn get_hostname() -> String {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::SystemInformation::GetComputerNameW;
        const MAX_NAME: usize = 256;
        let mut buf = [0u16; MAX_NAME];
        let mut len = MAX_NAME as u32;
        unsafe {
            if GetComputerNameW(buf.as_mut_ptr(), &mut len) != 0 {
                String::from_utf16_lossy(&buf[..len as usize])
            } else {
                "unknown".to_string()
            }
        }
    }
    #[cfg(not(windows))]
    {
        std::env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".to_string())
    }
}

pub fn get_username() -> String {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::SystemInformation::GetUserNameW;
        const UNLEN: usize = 256;
        let mut buf = [0u16; UNLEN];
        let mut len = UNLEN as u32;
        unsafe {
            if GetUserNameW(buf.as_mut_ptr(), &mut len) != 0 {
                String::from_utf16_lossy(&buf[..(len - 1) as usize])
            } else {
                "unknown".to_string()
            }
        }
    }
    #[cfg(not(windows))]
    {
        std::env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string())
    }
}

pub fn get_pid() -> u32 {
    std::process::id()
}

pub fn get_os() -> &'static str {
    "windows"
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
    // Connect a UDP socket to a public address to determine the local interface IP.
    // No traffic is actually sent.
    use std::net::UdpSocket;
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    Some(socket.local_addr().ok()?.ip().to_string())
}

pub fn get_privilege_info() -> String {
    #[cfg(windows)]
    {
        use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
        use windows_sys::Win32::Security::{
            GetTokenInformation, LookupPrivilegeNameW, TokenElevation, TokenPrivileges,
            TOKEN_ELEVATION, TOKEN_PRIVILEGES, TOKEN_QUERY,
        };
        use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

        let mut token: HANDLE = std::ptr::null_mut();
        if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) } == 0 {
            return "User".to_string();
        }

        // Check elevation
        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut size = 0u32;
        let is_elevated = unsafe {
            GetTokenInformation(
                token,
                TokenElevation,
                &mut elevation as *mut _ as *mut _,
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut size,
            )
        } != 0
            && elevation.TokenIsElevated != 0;

        // Get privilege list
        let mut priv_names: Vec<String> = Vec::new();

        // First call to get required buffer size
        let mut buf_size = 0u32;
        unsafe {
            GetTokenInformation(
                token,
                TokenPrivileges,
                std::ptr::null_mut(),
                0,
                &mut buf_size,
            );
        }

        if buf_size > 0 {
            let mut buffer = vec![0u8; buf_size as usize];
            let mut return_length = 0u32;
            let ok = unsafe {
                GetTokenInformation(
                    token,
                    TokenPrivileges,
                    buffer.as_mut_ptr() as *mut _,
                    buf_size,
                    &mut return_length,
                )
            };

            if ok != 0 {
                let tp = unsafe { &*(buffer.as_ptr() as *const TOKEN_PRIVILEGES) };
                let count = tp.PrivilegeCount;
                let entries = unsafe {
                    std::slice::from_raw_parts(
                        tp.Privileges.as_ptr(),
                        count as usize,
                    )
                };

                for entry in entries {
                    let mut name_len = 0u32;
                    unsafe {
                        LookupPrivilegeNameW(
                            std::ptr::null(),
                            &entry.Luid,
                            std::ptr::null_mut(),
                            &mut name_len,
                        );
                    }
                    if name_len > 0 {
                        let mut name_buf = vec![0u16; name_len as usize + 1];
                        let ok = unsafe {
                            LookupPrivilegeNameW(
                                std::ptr::null(),
                                &entry.Luid,
                                name_buf.as_mut_ptr(),
                                &mut name_len,
                            )
                        };
                        if ok != 0 {
                            let name = String::from_utf16_lossy(&name_buf[..name_len as usize]);
                            // Skip trivial privileges
                            if name != "SeChangeNotifyPrivilege" {
                                priv_names.push(name);
                            }
                        }
                    }
                }
            }
        }

        unsafe { CloseHandle(token) };

        let level = if !is_elevated {
            "User"
        } else {
            "Admin"
        };

        if priv_names.is_empty() {
            level.to_string()
        } else {
            format!("{}: {}", level, priv_names.join(", "))
        }
    }

    #[cfg(not(windows))]
    {
        "User".to_string()
    }
}
