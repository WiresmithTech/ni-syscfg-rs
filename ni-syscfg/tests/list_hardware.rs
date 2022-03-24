use ni_syscfg::SessionConfig;

#[test]
fn test_list_hardware() {
    let session = SessionConfig::new()
        .connect()
        .expect("Couldn't Open Session");

    for hardware in session.find_hardware(None).expect("Couldn't List Hardware") {
        let name = hardware.name().expect("Couldn't Get hardware Name");
        let bus_type = hardware
            .connects_to_bus_type()
            .expect("Couldnt get bus type");
        println!("Hardware found {name} connected to {bus_type:?}");
    }
}
