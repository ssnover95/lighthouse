// Save the bridge to file and load it back in;
fn main() {
    use lighthouse::*;

    let filename = "test_bridge";
    // Create bridge from an IP and a Key.
    let b = bridge::Bridge::new("127.0.0.1".parse().unwrap(), "<SOME-KEY>".to_owned()).unwrap();

    b.to_file(filename).expect("Could not write to file!");
    bridge::Bridge::from_file(filename).expect("Could not read from file!");
}
