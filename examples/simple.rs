use mac_address::{get_mac_address, get_mac_addresses};

fn main() {
    match get_mac_address() {
        Ok(Some(ma)) => {
            println!("MAC addr = {}", ma);
            println!("bytes = {:?}", ma.bytes());
        }
        Ok(None) => println!("No MAC address found."),
        Err(e) => println!("{:?}", e),
    }

    match get_mac_addresses() {
        Ok(mac_addresses) => {
            println!("\nList of all universal MAC addresses:");
            for (i, ma) in mac_addresses.iter().enumerate() {
                // Filtering out local MAC addresses
                if (ma.bytes()[0] & 2) != 2 {
                    println!("{}. {}", i+1, ma);
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }
}
