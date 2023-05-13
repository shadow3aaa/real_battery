use real_battery::{core::*, set_self_sched};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(s) = args.get(1) {
        if s == "--init_mount" {
            mount_init();
        }
    }
    test_support();
    set_self_sched();

    let list = [
        ("/sys/class/qcom-battery/fg1_rsoc", None),
        ("/sys/class/power_supply/bms/rsoc", None),
        (
            "/sys/class/power_supply/bms/capacity_raw",
            Some(|r: u32| r / 100),
        ),
    ];

    run(list);
    eprintln!("Unsupported device");
    std::process::exit(-1);
}
