use real_battery::{core::*, set_self_sched, test_path};
use std::env;

fn main() {
    let list = [
        ("/sys/class/qcom-battery/fg1_rsoc", None),
        ("/sys/class/power_supply/bms/rsoc", None),
        (
            "/sys/class/power_supply/bms/capacity_raw",
            Some(|r: u32| r / 100),
        ),
    ];
    
    if !test_support(list) {
        eprintln!("Unsupported device");
        std::process::exit(-1);
    }
    
    let args: Vec<String> = env::args().collect();
    if let Some(s) = args.get(1) {
        if s == "--init_mount" {
            mount_init();
        }
    }
    set_self_sched();
    run(list);
    std::process::exit(-1);
}

fn test_support<T, F>(list: T) -> bool
where
    T: IntoIterator<Item = (&'static str, Option<F>)>,
    F: Fn(u32) -> u32,
{
    let mut r = false;
    for (path, _) in list {
        if test_path(path) {
            r = true
        }
    }
    r
}