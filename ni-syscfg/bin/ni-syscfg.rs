use std::path::PathBuf;

use ni_syscfg::SessionConfig;

fn main() {
    let session = SessionConfig::new()
        .target("192.168.10.102")
        .username("admin")
        .unwrap()
        .password("labview")
        .unwrap()
        .connect()
        .expect("Session failed");
    session
        .get_software_image(&PathBuf::from(
            r"C:\Users\JamesMcNally\Documents\Delete\TestImage",
        ))
        .unwrap();
}
