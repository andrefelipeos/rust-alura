fn multiplication_table(number: u8) {
    if number > 25 {
        println!("We do not work with numbers greater than 25.");
        return;
    }

    for multiplier in 1..11 {
        println!("{:2.} x {:2.} = {:3.}", number, multiplier, number * multiplier);
    }
}

fn main() {
    multiplication_table(24);
}
