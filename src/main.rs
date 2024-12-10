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

fn main() {
    println!("{}", dec_to_bin(69));
}
