enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn run() {
    // Basic use of enums
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    route(four);
    route(six);

    // Basic use of match operator
    println!("{}", value_in_cents(Coin::Nickel));

    // Match with catch-all and underscore placeholder
    let dice_roll = 9;
    // Catch-all case
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    };
    // Underscore placeholder
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // If we wanted to do nothing, we would return an empty tuple `=> ()`
    };

    // If-let statements
    let config_max = Some(3u8);
    // Instead of doing this,
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    };
    // we can use an if-let statement
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        // The else part is equivalent to `_ => {}`
        // count += 1;
    }
}

fn route(ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn move_player(num_spaces: u8) {}

fn reroll() {}
