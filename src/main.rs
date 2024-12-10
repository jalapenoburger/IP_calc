fn dec_to_bin(mut dec: u32) -> String {
    let mut bin = String::new();

    // converting decimal to binary
    while dec > 0 {
        bin.insert(0, char::from_digit((dec % 2) as u32, 10).unwrap());
        dec /= 2;
    } // end while

    while bin.len() < 8 {
        bin.insert(0, '0');
    }

    bin
}

fn ip_address_to_bin(ip_address: &str) -> String {
    let parts: Vec<String> = ip_address
        .split(".")
        // using map to change elements of parts array from decimal values as strings to binary as u32 and into string again (I guess...)
        .map(|part| {
            let dec = part.parse::<u32>().unwrap();
            dec_to_bin(dec)
        }) // end map
        .collect();

    parts.join(".")
}

fn main() {
    println!("{}", ip_address_to_bin("69.69.69.69"));
}
