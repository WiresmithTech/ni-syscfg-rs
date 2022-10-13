use std::{
    io::{self, Write},
    path::PathBuf,
};

use ni_syscfg::software::{ImageInfo, NetworkInterfaceSettings};
use ni_syscfg::SessionConfig;

fn main() {
    let mut address = String::new();
    let mut user = String::new();
    let mut password = String::new();

    input("Enter the address", &mut address);
    input("Enter the username", &mut user);
    input("Enter the device password", &mut password);

    let session = SessionConfig::new()
        .target(&address)
        .username(&user)
        .unwrap()
        .password(&password)
        .unwrap()
        .connect()
        .expect("Session failed");

    println!("Preparing local files...");
    let unencrypted_image = prepare_image_path("test_unencrypted_image");
    let encrypted_image = prepare_image_path("test_encrypted_image");

    let image_data = ImageInfo {
        title: "test".to_owned(),
        id: "".to_owned(),
        description: "Test Image".to_owned(),
        version: "1.0".to_owned(),
    };

    //Lets go for an unencrypted first.
    println!("Getting unencrypted image...");
    session
        .get_system_image(
            &unencrypted_image,
            &image_data,
            None,
            &["/c", "/C"],
            true,
            false,
        )
        .expect("Get Unencrypted Image Failed");
    println!("Setting unencrypted image...");
    session
        .set_system_image(
            &unencrypted_image,
            None,
            &["/c", "/C"],
            true,
            false,
            NetworkInterfaceSettings::PreservePrimaryPreserveOthers,
        )
        .expect("Set Unecrypted Image Failed");

    println!("Checking it doesn't overwrite if it exists.");
    let result = session.get_system_image(
        &unencrypted_image,
        &image_data,
        None,
        &["/c", "/C"],
        true,
        false,
    );
    assert!(result.is_err());

    // Now encrpyted
    println!("Getting encrypted image...");
    session
        .get_system_image(
            &encrypted_image,
            &image_data,
            Some("test123"),
            &["/c", "/C"],
            true,
            false,
        )
        .expect("Get Encrypted Image Failed");
    println!("Setting encrypted image...");
    session
        .set_system_image(
            &encrypted_image,
            Some("test123"),
            &["/c", "/C"],
            true,
            false,
            NetworkInterfaceSettings::PreservePrimaryPreserveOthers,
        )
        .expect("Set encrypted Image Failed");

    println!("Success!");
}

fn input(prompt: &str, output: &mut String) {
    print!("{prompt}: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(output).unwrap();

    //remove new line
    output.pop();
    #[cfg(target_os = "windows")]
    output.pop();
}

fn prepare_image_path(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(name);
    if path.exists() {
        std::fs::remove_dir_all(&path).unwrap();
    }
    return path;
}
