use real_battery::{core::*, set_self_sched};

fn main() {
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
