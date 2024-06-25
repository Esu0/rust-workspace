use opt_div::FromStrFast;

fn main() {
    println!("{}", u32::parse_fast("12345678").unwrap());
    println!("{}", u32::parse_fast("1234567890").unwrap());
    println!("{}", u32::parse_fast("0").unwrap());
}
