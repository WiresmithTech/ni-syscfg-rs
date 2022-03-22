use ni_syscfg::Session;

#[test]
fn test_list_hardware() {
    let session = Session::new_local_session().expect("Couldn't Open Session");

    for hardware in session.find_hardware().expect("Couldn't List Hardware") {
        let name = hardware.get_name().expect("Couldn't Get hardware Name");
        println!("Hardware found {name}");
    }
}
