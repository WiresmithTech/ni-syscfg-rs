use ni_syscfg::{ExpertType, FilterMode, SessionConfig};

#[test]
fn test_list_hardware() {
    let session = SessionConfig::new()
        .connect()
        .expect("Couldn't Open Session");

    for hardware in session
        .find_hardware(None, Some(&[ExpertType::NiDaqmx]))
        .expect("Couldn't List Hardware")
    {
        let name = hardware.name().expect("Couldn't Get hardware Name");
        let bus_type = hardware
            .connects_to_bus_type()
            .expect("Couldnt get bus type");
        println!("Hardware found {name} connected to {bus_type:?}");
    }
}

#[test]
fn test_list_hardware_with_filter() {
    let session = SessionConfig::new().connect().unwrap();
    let mut filter = session.create_filter().unwrap();
    filter.set_mode(FilterMode::MatchValuesAny);
    let hardware_list = session.find_hardware(Some(&filter), None).unwrap();

    for hardware in hardware_list {
        println!("Found {}", hardware.name().unwrap())
    }
}
