use std::io::{self, Read};

enum Discount {
    Percentage(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    const FLAT_DISCOUNT: Discount = Discount::Flat(5);
    println!(
        "The type of FLAT_DISCOUNT is: {}",
        std::any::type_name::<Discount>()
    );

    match FLAT_DISCOUNT {
        Discount::Flat(5) => println!("The discount is 5"),
        Discount::Flat(other) => println!("{:?}", other),
        _ => println!("not found"),
    }

    let mut input_event = String::new();
    let mut input_price = String::new();

    println!("Please enter the event name");
    io::stdin()
        .read_line(&mut input_event)
        .expect("Failed to read input");

    println!("Please enter ticket price");
    io::stdin()
        .read_line(&mut input_price)
        .expect("Failed to read price");
    let price = input_price.trim().parse().expect("bad input");

    let ticket_1 = Ticket {
        event: input_event.trim().to_owned(),
        price: price,
    };

    match ticket_1 {
        Ticket { price: 50, .. } => println!("price is{:?}", ticket_1.price),
        Ticket { price, event } => println!("price is {:?} for event {:?}", price, event),
    }
}
