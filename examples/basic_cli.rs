fn main() {
    use lighthouse::*;
    use std::env;

    // Create bridge from an IP and a Key.
    let mut b = bridge::Bridge::new("<SOME-IP>".parse().unwrap(), "<SOME-KEY>").unwrap();

    // See if the user passed on or of
    let on_off = match env::args().collect::<Vec<String>>()[1] {
        "on" => true,
        "off" => false,
        _ => panic!("Unknown command. Use: on / off"),
    };
    let s = state!(on: on_off , bri:254);
    b.state_to_multiple(vec![1, 2, 3], vec![s; 3])
        .expect("Could not send all states!");
}
