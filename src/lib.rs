pub mod core;
pub mod mount;

pub fn set_self_sched() {
    let self_pid = &std::process::id().to_string();
    write_file(self_pid, "/dev/cpuset/background/tasks");
}

pub fn write_file(content: &str, path: &str) {
    use std::{
        fs::{set_permissions, OpenOptions},
        io::Write,
        os::unix::fs::PermissionsExt,
    };

    // debug
    // println!("path: {}, value: {}", path, content);

    match set_permissions(path, PermissionsExt::from_mode(0o644)) {
        Ok(()) => {
            match OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(path)
            {
                Ok(mut file) => match file.write_all(content.as_bytes()) {
                    Ok(()) => {}
                    Err(e) => eprintln!("Write failed: {}", e),
                },
                Err(e) => eprintln!("Open failed: {}", e),
            }
        }
        Err(e) => eprintln!("Set permissions failed: {}", e),
    }
}

pub fn is_writable(path: &str) -> bool {
    use std::fs;
    if let Ok(metadata) = fs::metadata(path) {
        return !metadata.permissions().readonly();
    }
    false
}

pub fn test_path(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

pub fn set_security_context(path: &str, context: &str) {
    let _ = exec_cmd("chcon", &[context, path]);
}

pub fn exec_cmd(command: &str, args: &[&str]) -> Result<String, i32> {
    use std::process::Command;
    let output = Command::new(command).args(args).output();

    match output {
        Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
        Err(e) => {
            eprintln!("{}", e);
            Err(-1)
        }
    }
}

use std::fs::File;
fn create_file(path: &str) -> std::io::Result<()> {
    let _file = File::create(path)?;
    Ok(())
}
