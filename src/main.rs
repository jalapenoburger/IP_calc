use std::io::{self, Write};
use std::net::Ipv4Addr;

fn count_where_0(ip: Ipv4Addr) -> i32 {
    let bit_ip = ip.to_bits();

    let binary = format!("{:08b}", bit_ip);
    let mut position = 0;

    for ch in binary.chars() {
        if ch == '0' {
            return position;
        }
        else {
            position += 1;
        }
    }
    0
}

fn ipv4_to_cidr(ip: Ipv4Addr) -> u8 {
    let octets = ip.octets();

    let mut cidr = 0;
    for &octet in &octets {
        cidr += octet.count_ones();
    }

    cidr as u8
}

fn to_binary_string_with_bar(ip: Ipv4Addr, pos: i32) -> String {
    let bit_ip = ip.to_bits();
    let binary = format!("{:032b}", bit_ip);
    let mut result = String::new();

    for (i, ch) in binary.chars().enumerate() {
        if i == pos as usize && pos != 0{
            result.push_str("|");
        }
        if i % 8 == 0 && i != 0 {
            result.push('.');
        }
        result.push(ch);
    }

    result
}

fn network_address(ip: Ipv4Addr, pos: i32) -> (String, Ipv4Addr) {
    let bit_ip = ip.to_bits();
    let mask = u32::MAX << (32 - pos);
    let result = Ipv4Addr::from_bits(bit_ip & mask);

    (to_binary_string_with_bar(result, pos), result)
}

fn octets_check_fill_ones(mut ip: [u8; 4], pos: usize) -> [u8; 4]{
    let byte_index = pos / 8;
    let bit_index = pos % 8;

    if byte_index < ip.len() {
        ip[byte_index] |= 0xFF >> bit_index;
    }

    for i in (byte_index + 1)..ip.len() {
        ip[i] = 0xFF;
    }
    ip
}

fn main() {
    print!("Enter an IPv4 address: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!! >:(");

    let input = input.trim();
    let ipv4: Ipv4Addr = match input.parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Invalid IPv4 address.");
            return;
        }
    };
    print!("Enter a subnet mask: ");
    io::stdout().flush().unwrap();

    let mut mask_input = String::new();
    io::stdin()
        .read_line(&mut mask_input)
        .expect("Failed to read line!!");
    let mask_input = mask_input.trim();

    let mask = if let Ok(prefix_len) = mask_input.parse::<u8>() {
        if prefix_len <= 32 {
            // Calculate subnet mask based on prefix length
            let mut mask = 0u32;
            for i in 0..prefix_len {
                mask |= 1 << (31 - i);
            }
            // Convert the mask to Ipv4Addr
            Ipv4Addr::new(
                ((mask >> 24) & 0xFF) as u8,
                ((mask >> 16) & 0xFF) as u8,
                ((mask >> 8) & 0xFF) as u8,
                (mask & 0xFF) as u8,
            )
        } else {
            eprintln!("Invalid subnet prefix length.");
            return;
        }
    } else {
        match mask_input.parse::<Ipv4Addr>() {
            Ok(mask) => mask,
            Err(_) => {
                eprintln!("Invalid subnet mask format.");
                return;
            }
        }
    };

    let split_pos = count_where_0(mask);
    let mask_number = ipv4_to_cidr(mask);

    println!();
    println!("IPv4 Address: {} ({})", ipv4, to_binary_string_with_bar(ipv4, split_pos));
    println!("Subnet Mask: {} = {} ({})", mask, mask_number, to_binary_string_with_bar(mask, split_pos));

    if mask_number == 32 {return;}

    let network_string = network_address(ipv4, split_pos).0;
    let network_adr = network_address(ipv4, split_pos).1;

    let mut broadcast_address_octets = network_adr.octets();
    broadcast_address_octets = octets_check_fill_ones(broadcast_address_octets, split_pos as usize);
    let broadcast_address = Ipv4Addr::from(broadcast_address_octets);

    let mut host_min = network_adr.octets();
    host_min[3] += 1;
    let host_min = Ipv4Addr::from(host_min);

    broadcast_address_octets[3] -= 1;
    let host_max = Ipv4Addr::from(broadcast_address_octets);

    println!("Network Address: {}/{} ({})", network_adr, mask_number, network_string);
    println!("Broadcast Address: {} ({})", broadcast_address, to_binary_string_with_bar(broadcast_address, split_pos));
    println!("Hosts: {} ", 2_i32.pow((32 - mask_number) as u32) - 2);

    if mask_number == 31 {return;}

    println!("Host min: {} ({})", host_min, to_binary_string_with_bar(host_min, split_pos));
    println!("Host max: {} ({})", host_max, to_binary_string_with_bar(host_max, split_pos));

    println!("Press enter to close. ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}
