fn can_or_must_vote(name: &String, age: u8, literate: bool) {
    let mut can_vote: bool = false;
    let mut must_vote: bool = false;
    if age >= 16 {
        can_vote = true;
    }
    if age >= 18 && age <= 70 && literate {
        must_vote = true;
    }

    if must_vote {
        println!("{} must vote.", name);
    } else if can_vote {
        println!("{} can vote.", name);
    } else {
        println!("{} cannot vote.", name);
    }
}

fn can_have_a_driver_license(name: &String, age: u8, literate: bool) {
    if age >= 18 && literate {
        println!("{} can have a driver's license.", name);
    } else {
        println!("{} cannot have a driver's license.", name);
    }
}

fn can_buy_and_drink_alcohol(name: &String, age: u8) {
    if age >= 18 {
        println!("{} can buy and drink alcoholic beverage.", name);
    } else {
        println!("{} cannot buy or drink alcoholic beverage.", name);
    }
}

fn must_enlist_for_military_service(name: &String, sex: &String, age: u8) {
    if sex == "male" {
        if age < 18 {
            println!("{} does not need to enlist himself yet.", name);
        } else if age == 18 {
            println!("{} must present himself for enlistment.", name);
        } else {
            println!("{} should have already been enlisted.", name);
        }
    } else {
        println!("{} does not need to enlist herself.", name);
    }
}

fn duties_and_rights(name: &String, sex: &String, age: u8, literate: bool) {
    can_or_must_vote(name, age, literate);
    can_have_a_driver_license(name, age, literate);
    can_buy_and_drink_alcohol(name, age);
    must_enlist_for_military_service(name, sex, age);
}

fn main() {
    let mut name: String;
    let mut sex: String;
    let mut age: u8;
    let mut literate: bool;

    name = String::from("Anna");
    sex = String::from("female");
    age = 19;
    literate = true;
    println!("\nname: {}\nsex: {}\nage: {}\nliterate: {}.", name, sex, age, literate);
    duties_and_rights(&name, &sex, age, literate);

    name = String::from("Abigail");
    sex = String::from("female");
    age = 72;
    literate = true;
    println!("\nname: {}\nsex: {}\nage: {}\nliterate: {}.", name, sex, age, literate);
    duties_and_rights(&name, &sex, age, literate);

    name = String::from("Samwell");
    sex = String::from("male");
    age = 34;
    literate = false;
    println!("\nname: {}\nsex: {}\nage: {}\nliterate: {}.", name, sex, age, literate);
    duties_and_rights(&name, &sex, age, literate);

    name = String::from("John");
    sex = String::from("male");
    age = 16;
    literate = true;
    println!("\nname: {}\nsex: {}\nage: {}\nliterate: {}.", name, sex, age, literate);
    duties_and_rights(&name, &sex, age, literate);
}
