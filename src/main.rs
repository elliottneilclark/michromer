extern crate michromer;

use michromer::client::Client;

fn get_key() -> String {
    return option_env!("SF_KEY")
               .expect("Need to set SF_KEY")
               .to_string();
}

fn main() {
    let key = get_key();
    let mut client = Client::new(&key);
    let level = client.start_level("chock_a_block").unwrap();
    println!("{:?}", level);
}
