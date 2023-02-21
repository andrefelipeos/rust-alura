const PI: f64 = 3.14159;

fn area_of_disk(radius: f64) -> f64 {
    PI * radius * radius
}

fn circumference_of_circle(radius: f64) -> f64 {
    2.0 * PI * radius
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 1.8 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

fn main() {
    let mut radius: f64;

    radius= 4.37;
    println!("A disk with a radius of {:.2} meters has an area of {:.2} square meters and a circumference of {:.2} meters.", radius, area_of_disk(radius), circumference_of_circle(radius));

    radius= 6.125;
    println!("A disk with a radius of {:.2} meters has an area of {:.2} square meters and a circumference of {:.2} meters.", radius, area_of_disk(radius), circumference_of_circle(radius));

    let mut degrees: f32;

    degrees = 10.2;
    println!("{:.2} °C is equals to {:.2} °F.", degrees, celsius_to_fahrenheit(degrees));
    println!("{:.2} °F is equals to {:.2} °C.", degrees, fahrenheit_to_celsius(degrees));

    degrees = 70.4;
    println!("{:.2} °C is equals to {:.2} °F.", degrees, celsius_to_fahrenheit(degrees));
    println!("{:.2} °F is equals to {:.2} °C.", degrees, fahrenheit_to_celsius(degrees));
}
