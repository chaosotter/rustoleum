fn main() {
    print!("    ");
    for b in 0x0..=0xf {
        print!("{:3x}", b);
    }
    println!();
    println!("   +-------------------------------------------------");
    for a in 0x0..=0xf {
        print!("{:2x} |", a);
        for b in 0x0..=0xf {
            print!("{:3x}", a*b);
        }
        println!();
    }
}
