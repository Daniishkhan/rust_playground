// this is a program to practice advanced rust

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

    let ticket_1 = Ticket {
        event: "music".to_owned(),
        price: 51,
    };

    match ticket_1 {
        Ticket { price: 50, .. } => println!("price is{:?}", ticket_1.price),
        Ticket { price, event } => println!("price is {:?} for event {:?}", price, event),
    }
}
