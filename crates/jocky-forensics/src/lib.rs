use jocky_common::Evidence;

pub trait Collector {
    fn collect(&self) -> Result<Vec<Evidence>, String>;
}

pub struct CollectorRegistry;

impl CollectorRegistry {
    pub fn get_system_collector() -> Box<dyn Collector> {
        #[cfg(target_os = "windows")]
        return Box::new(windows::WindowsSystemCollector::new());
        #[cfg(target_os = "linux")]
        return Box::new(linux::LinuxSystemCollector::new());
        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        return Box::new(mock::MockSystemCollector::new());
    }

    pub fn get_process_collector() -> Box<dyn Collector> {
        #[cfg(target_os = "windows")]
        return Box::new(windows::WindowsProcessCollector::new());
        #[cfg(target_os = "linux")]
        return Box::new(linux::LinuxProcessCollector::new());
        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        return Box::new(mock::MockProcessCollector::new());
    }

    pub fn get_network_collector() -> Box<dyn Collector> {
        #[cfg(target_os = "windows")]
        return Box::new(windows::WindowsNetworkCollector::new());
        #[cfg(target_os = "linux")]
        return Box::new(linux::LinuxNetworkCollector::new());
        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        return Box::new(mock::MockNetworkCollector::new());
    }
}

pub mod mock {
    use super::*;
    pub struct MockSystemCollector;
    impl MockSystemCollector {
        pub fn new() -> Self {
            Self
        }
    }
    impl Collector for MockSystemCollector {
        fn collect(&self) -> Result<Vec<Evidence>, String> {
            Ok(vec![])
        }
    }

    pub struct MockProcessCollector;
    impl MockProcessCollector {
        pub fn new() -> Self {
            Self
        }
    }
    impl Collector for MockProcessCollector {
        fn collect(&self) -> Result<Vec<Evidence>, String> {
            Ok(vec![Evidence::new(
                "HOST-001",
                "process",
                "mock",
                serde_json::json!({
                    "pid": 4210,
                    "name": "example.exe",
                    "parent_pid": 2100
                }),
            )])
        }
    }

    pub struct MockNetworkCollector;
    impl MockNetworkCollector {
        pub fn new() -> Self {
            Self
        }
    }
    impl Collector for MockNetworkCollector {
        fn collect(&self) -> Result<Vec<Evidence>, String> {
            Ok(vec![])
        }
    }
}

#[cfg(target_os = "windows")]
pub mod windows {
    use super::*;
    use sysinfo::System;

    pub struct WindowsSystemCollector;
    impl WindowsSystemCollector {
        pub fn new() -> Self {
            Self
        }
    }
    impl Collector for WindowsSystemCollector {
        fn collect(&self) -> Result<Vec<Evidence>, String> {
            let mut sys = System::new_all();
            sys.refresh_all();

            let host = System::host_name().unwrap_or_else(|| "UNKNOWN".to_string());
            let os = System::name().unwrap_or_else(|| "Windows".to_string());
            let version = System::os_version().unwrap_or_default();
            let arch = System::cpu_arch().unwrap_or_default();

            let ev = Evidence::new(
                &host,
                "system",
                "windows",
                serde_json::json!({
                    "hostname": host,
                    "os": os,
                    "version": version,
                    "architecture": arch,
                }),
            );

            Ok(vec![ev])
        }
    }

    pub struct WindowsProcessCollector;
    impl WindowsProcessCollector {
        pub fn new() -> Self {
            Self
        }
    }
    impl Collector for WindowsProcessCollector {
        fn collect(&self) -> Result<Vec<Evidence>, String> {
            let mut sys = System::new_all();
            sys.refresh_processes();
            let host = System::host_name().unwrap_or_else(|| "UNKNOWN".to_string());

            let mut evidences = Vec::new();
            for (pid, process) in sys.processes() {
                evidences.push(Evidence::new(
                    &host,
                    "process",
                    "windows",
                    serde_json::json!({
                        "pid": pid.as_u32(),
                        "name": process.name(),
                        "parent_pid": process.parent().map(|p| p.as_u32()),
                        "path": process.exe().map(|p| p.to_string_lossy().to_string()),
                    }),
                ));
            }
            Ok(evidences)
        }
    }

    pub struct WindowsNetworkCollector;
    impl WindowsNetworkCollector {
        pub fn new() -> Self {
            Self
        }
    }
    impl Collector for WindowsNetworkCollector {
        fn collect(&self) -> Result<Vec<Evidence>, String> {
            // Placeholder: true network connection parsing usually requires netstat/Windows API
            // For phase 2 demonstration we return simulated connections for existing processes
            let host = System::host_name().unwrap_or_else(|| "UNKNOWN".to_string());
            let ev = Evidence::new(
                &host,
                "network",
                "windows",
                serde_json::json!({
                    "pid": 4210,
                    "local_ip": "192.168.1.50",
                    "local_port": 54321,
                    "remote_ip": "203.0.113.10",
                    "remote_port": 443,
                    "state": "ESTABLISHED",
                    "protocol": "TCP"
                }),
            );
            Ok(vec![ev])
        }
    }
}

#[cfg(target_os = "linux")]
pub mod linux {
    use super::*;
    use sysinfo::System;

    pub struct LinuxSystemCollector;
    impl LinuxSystemCollector {
        pub fn new() -> Self {
            Self
        }
    }
    impl Collector for LinuxSystemCollector {
        fn collect(&self) -> Result<Vec<Evidence>, String> {
            let mut sys = System::new_all();
            sys.refresh_all();

            let host = System::host_name().unwrap_or_else(|| "UNKNOWN".to_string());
            let os = System::name().unwrap_or_else(|| "Linux".to_string());
            let version = System::os_version().unwrap_or_default();
            let arch = System::cpu_arch().unwrap_or_default();

            let ev = Evidence::new(
                &host,
                "system",
                "linux",
                serde_json::json!({
                    "hostname": host,
                    "os": os,
                    "version": version,
                    "architecture": arch,
                }),
            );

            Ok(vec![ev])
        }
    }

    pub struct LinuxProcessCollector;
    impl LinuxProcessCollector {
        pub fn new() -> Self {
            Self
        }
    }
    impl Collector for LinuxProcessCollector {
        fn collect(&self) -> Result<Vec<Evidence>, String> {
            let mut sys = System::new_all();
            sys.refresh_processes();
            let host = System::host_name().unwrap_or_else(|| "UNKNOWN".to_string());

            let mut evidences = Vec::new();
            for (pid, process) in sys.processes() {
                evidences.push(Evidence::new(
                    &host,
                    "process",
                    "linux",
                    serde_json::json!({
                        "pid": pid.as_u32(),
                        "name": process.name(),
                        "parent_pid": process.parent().map(|p| p.as_u32()),
                        "path": process.exe().map(|p| p.to_string_lossy().to_string()),
                    }),
                ));
            }
            Ok(evidences)
        }
    }

    pub struct LinuxNetworkCollector;
    impl LinuxNetworkCollector {
        pub fn new() -> Self {
            Self
        }
    }
    impl Collector for LinuxNetworkCollector {
        fn collect(&self) -> Result<Vec<Evidence>, String> {
            // For phase 2 demonstration we return simulated connection
            let host = System::host_name().unwrap_or_else(|| "UNKNOWN".to_string());
            let ev = Evidence::new(
                &host,
                "network",
                "linux",
                serde_json::json!({
                    "pid": 4210,
                    "local_ip": "192.168.1.50",
                    "local_port": 54321,
                    "remote_ip": "203.0.113.10",
                    "remote_port": 443,
                    "state": "ESTABLISHED",
                    "protocol": "TCP"
                }),
            );
            Ok(vec![ev])
        }
    }
}
