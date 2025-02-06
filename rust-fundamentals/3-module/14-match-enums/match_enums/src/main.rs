enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes((size as f64) / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes((size as f64) / 1_000_000.0),
        _ => FileSize::Gigabytes((size as f64) / 1_000_000_000.0),
    };
    /*
    The as keyword in Rust is used for explicit type conversion (casting). In your code, it is used to convert size from u64 to f64 in order to perform floating-point division.

    In Rust, arithmetic operations between different numeric types are not allowed without explicit conversion. This helps prevent unintended precision loss or truncation. In this case, the size variable is a u64, which is an unsigned 64-bit integer, and 1000.0 is a floating-point number. To perform division between these two types, you need to convert size to a floating-point number using the as keyword.
    */

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}

fn main() {
    let result = format_size(6888837399);
    println!("{}", result)
}
