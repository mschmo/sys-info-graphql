extern crate juniper;
extern crate sys_info;
extern crate libc;

use self::libc::timeval;
use self::juniper::Context;


pub enum ByteUnit {
    KB,
    MB,
    GB,
    TB
}

pub enum CycleUnit {
    MHz,
    GHz
}

fn get_byte_conversion(unit: Option<ByteUnit>) -> u64 {
    match unit {
        Some(u) => match u {
            ByteUnit::KB => 1,
            ByteUnit::MB => 1_000,
            ByteUnit::GB => 1_000_000,
            ByteUnit::TB => 1_000_000_000
        },
        None => 1
    }
}

fn get_cycle_conversion(unit: Option<CycleUnit>) -> u64 {
    match unit {
        Some(u) => match u {
            CycleUnit::MHz => 1,
            CycleUnit::GHz => 1_000
        },
        None => 1
    }
}

fn convert_to_string(value: u64, conversion_rate: u64) -> String {
    if conversion_rate == 0 {
        return String::from("0");
    }
    let result = value as f32 / conversion_rate as f32;
    format!("{:.2}", result)
}

pub struct DiskInformation {
    pub total: String,
    pub free: String
}

pub struct LoadAverage {
    pub one: String,
    pub five: String,
    pub fifteen: String
}

pub struct MemoryInformation {
    pub total: String,
    pub free: String,
    pub available: String,
    pub buffers: String,
    pub cached: String,
    pub swap_total: String,
    pub swap_free: String
}

pub struct System {
    pub os_type: String,
    pub os_release: String,
    pub hostname: String,
    pub cpu_speed: u64,
    pub cpu_num: String
}

impl System {
    pub fn new() -> System {
        System {
            os_type: match sys_info::os_type() {
                Ok(r) => r,
                Err(_) => String::from("Unsupported Operating System")
            },
            os_release: match sys_info::os_release() {
                Ok(r) => r,
                Err(_) => String::from("Failed to get OS release version")
            },
            hostname: match sys_info::hostname() {
                Ok(r) => r,
                Err(_) => String::from("Failed to get host name")
            },
            cpu_speed: match sys_info::cpu_speed() {
                Ok(r) => r,
                Err(_) => 0
            },
            cpu_num: match sys_info::cpu_num() {
                Ok(r) => r.to_string(),
                Err(_) => String::from("Failed to get CPU count")
            }
        }
    }

    pub fn get_disk_info(&self, unit: Option<ByteUnit>) -> DiskInformation {
        let disk = match sys_info::disk_info() {
            Ok(result) => result,
            Err(_) => panic!("Crap.")
        };
        let conversion_rate = get_byte_conversion(unit);
        DiskInformation {
            total: convert_to_string(disk.total, conversion_rate),
            free: convert_to_string(disk.free, conversion_rate)
        }
    }

    pub fn get_load_average(&self) -> LoadAverage {
        match sys_info::loadavg() {
            Ok(la) => LoadAverage {
                one: la.one.to_string(),
                five: la.five.to_string(),
                fifteen: la.fifteen.to_string()
            },
            Err(_) => panic!("Crap.")
        }
    }

    pub fn get_memory_information(&self, unit: Option<ByteUnit>) -> MemoryInformation {
        let mem_info = match sys_info::mem_info() {
            Ok(result) => result,
            Err(_) => panic!("Crap.")
        };
        let conversion = get_byte_conversion(unit);
        MemoryInformation {
            total: convert_to_string(mem_info.total, conversion),
            free: convert_to_string(mem_info.free, conversion),
            available: convert_to_string(mem_info.avail, conversion),
            buffers: convert_to_string(mem_info.buffers, conversion),
            cached: convert_to_string(mem_info.cached, conversion),
            swap_total: convert_to_string(mem_info.swap_total, conversion),
            swap_free: convert_to_string(mem_info.swap_free, conversion)
        }
    }

    pub fn get_cpu_speed(&self, unit: Option<CycleUnit>) -> String {
        convert_to_string(self.cpu_speed, get_cycle_conversion(unit))
    }

    pub fn get_proc_total(&self) -> String {
        // Not supported on Windows
        match sys_info::proc_total() {
            Ok(result) => result.to_string(),
            Err(_) => String::from("Failed to get proccess quantity.")
        }
    }

    pub fn get_boot_time(&self) -> String {
        let boot_time = match sys_info::boottime() {
            Ok(r) => r,
            Err(_) => timeval {
                tv_sec: 0,
                tv_usec: 0
            }
        };
        format!("{} {}", boot_time.tv_sec, boot_time.tv_usec)
    }
}

impl Context for System {}
