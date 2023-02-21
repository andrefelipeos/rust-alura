fn multiplication_table(number: u8) {
    if number > 25 {
        println!("We do not work with numbers greater than 25.");
        return;
    }

    for multiplier in 1..11 {
        println!("        {:2.} x {:2.} = {:3.}", number, multiplier, number * multiplier);
    }
}

fn print_a_pyramid(height: u8) {
    if height > 38 {
        println!("We cannot stack so many stones, sorry!");
    }

    for level in 1..=height {
        print!("  {:2.}:   ", level);
        for _spot in 1..=(height - level) {
            print!(" ");
        }
        for _spot in (height - level + 1)..=height {
            print!("x");
        }
        for _spot in (height + 1)..(height + level) {
            print!("x");
        }
        println!();
    }
}

fn main() {
    let number: u8 = 24;
    println!("-\nHere is the multiplication table for {}:\n", number);
    multiplication_table(number);

    println!();

    let levels: u8 = 8;
    println!("-\nNow, here is a pyramid with {} levels:\n", levels);
    print_a_pyramid(levels);

    println!();
}
