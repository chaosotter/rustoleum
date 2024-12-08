fn main() {
    let data: u64 = 0x7e81a581bd99817e;
    for r in (0..8).rev() {
        for c in 0..8 {
            if data & (1 << (r * 8 + c)) != 0 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
