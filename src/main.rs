use std::io;

#[allow(dead_code)]
#[derive(Debug)]
struct Sizes {
    bytes: i64,
    kilobytes: i64,
    megabytes: i64,
    gigabytes: i64,
    terabytes: i64,
}

impl Sizes {
    pub fn new(bytes: i64) -> Self {
        Sizes {
            bytes,
            kilobytes: bytes / 1000,
            megabytes: bytes / 1_000_000,
            gigabytes: bytes / 1_000_000_000,
            terabytes: bytes / 1_000_000_000_000,
        }
    }

    fn to_bytes(qty: i64, unit: &str) -> i64 {
        match unit.to_lowercase().as_str() {
            "b" | "bytes" => qty,
            "kb" | "kilobytes" => qty * 1_000,
            "mb" | "megabytes" => qty * 1_000_000,
            "gb" | "gigabytes" => qty * 1_000_000_000,
            "tb" | "terabytes" => qty * 1_000_000_000_000,
            _ => panic!("Unknown unit"),
        }
    }
}

fn main() {
    println!("Input your file size (quantity followed by unit, e.g., 10 MB): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut parts = input.split_whitespace();
    let quantity_str = parts.next().expect("Missing quantity");
    let unit_str = parts.next().expect("Missing unit");
    let quantity: i64 = quantity_str.parse().expect("Invalid quantity");

    print!("{}", unit_str);
    println!("{}", quantity);

    let bytes_qty = Sizes::to_bytes(quantity, unit_str);
    println!("Bytes: {}", bytes_qty);
    let sizes = Sizes::new(bytes_qty);
    println!("{:#?}", sizes);
}
