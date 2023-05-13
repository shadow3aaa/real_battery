use crate::mount::mount_bind;
use crate::{exec_cmd, is_writable, set_security_context, test_path, write_file};
use std::{fs::read_to_string, thread::sleep, time::Duration};

pub const BMS_CAPACITY: &str = "/sys/class/power_supply/bms/capacity";
pub const BAT_CAPACITY: &str = "/sys/class/power_supply/battery/capacity";
pub const MOUNT_CAPACITY: &str = "/cache/real_battery_cap";

pub fn run<T, F>(real_list: T)
where
    T: IntoIterator<Item = (&'static str, Option<F>)>,
    F: Fn(u32) -> u32,
{
    for (path, do_with) in real_list {
        if !test_path(path) {
            continue;
        }
        loop {
            let real: u32 = read_to_string(path).unwrap().trim().parse().unwrap();
            let real = match &do_with {
                Some(f) => f(real),
                None => real,
            };
            set_cap(real);
            sleep(Duration::from_secs(5));
        }
    }
}

pub fn test_support() {
    use std::process::exit;
    if !test_path(BMS_CAPACITY) {
        exit(-1);
    }
}

fn set_cap(cap: u32) {
    match is_writable(BMS_CAPACITY) {
        true => set_cap_by_write(cap),
        false => set_cap_by_mount(cap),
    }
}

fn set_cap_by_write(cap: u32) {
    write_file(&cap.to_string(), BMS_CAPACITY);
}

pub fn mount_init() {
    set_enforce(false);
    create_file(MOUNT_CAPACITY).unwrap();
    write_file("50", MOUNT_CAPACITY);
    mount_bind(MOUNT_CAPACITY, BAT_CAPACITY).unwrap();
    set_security_context(BAT_CAPACITY, "u:object_r:vendor_sysfs_battery_supply:s0");
    set_enforce(true);
    // 完成init操作直接退出
    std::process::exit(0);
}

fn set_cap_by_mount(cap: u32) {
    if !test_path(MOUNT_CAPACITY) {
        std::process::exit(-1);
    }
    write_file(&cap.to_string(), MOUNT_CAPACITY);
}

fn set_enforce(status: bool) {
    match status {
        true => {
            let _ = exec_cmd("setenforce", &["1"]);
        }
        false => {
            let _ = exec_cmd("setenforce", &["0"]);
        }
    }
}
