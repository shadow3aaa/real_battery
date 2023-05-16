use crate::mount::mount_bind;
use crate::{create_file, exec_cmd, is_writable, set_security_context, test_path, write_file};
use std::{error::Error, fs::read_to_string, thread::sleep, time::Duration};

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

fn set_cap(cap: u32) {
    match is_writable(BMS_CAPACITY) {
        true => set_cap_by_write(cap),
        false => set_cap_by_mount(cap),
    }
}

fn set_cap_by_write(cap: u32) {
    write_file(&cap.to_string(), BMS_CAPACITY);
}

fn mount_init() -> Result<(), Box<dyn Error>> {
    set_enforce(false);
    create_file(MOUNT_CAPACITY)?;
    let cur_cap = read_to_string(BAT_CAPACITY).unwrap_or("50".to_string());
    write_file(&cur_cap, MOUNT_CAPACITY);
    mount_bind(MOUNT_CAPACITY, BAT_CAPACITY)?;
    set_security_context(BAT_CAPACITY, "u:object_r:vendor_sysfs_battery_supply:s0");
    set_enforce(true);
    Ok(())
}

pub fn init_mount_until_success() {
    if !test_path(BMS_CAPACITY) && mount_init().is_err() {
        init_mount_until_success();
    }
}

fn set_cap_by_mount(cap: u32) {
    if !test_path(MOUNT_CAPACITY) {
        eprintln!("mount path: {} not found!", MOUNT_CAPACITY)
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
