fn divider() {
    println!("------------------------");
    println!();
}

fn quiz() {
    println!("QUIZ!");
}

fn main() {
    println!("Enums and Pattern Matching");
    divider();

    quiz();
    divider();

    defining_an_enum();
    divider();

    match_control_flow();
    divider();
}

fn defining_an_enum() {
    println!("Defining an Enum");

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {
        println!("{:?}", ip_kind);
    }

    route(four);
    route(six);

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home: {:?}", home);
    println!("home.kind: {:?}", home.kind);
    println!("home.address: {}", home.address);
    println!();
    println!("loopback: {:?}", loopback);
    println!("loopback.kind: {:?}", loopback.kind);
    println!("loopback.address: {}", loopback.address);
    println!();

    #[derive(Debug)]
    enum IpAddrV2 {
        V4(String),
        V6(String),
    }

    let home2 = IpAddrV2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddrV2::V6(String::from("::1"));

    println!("home2: {:?}", home2);
    println!("loopback2: {:?}", loopback2);

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn match_control_flow () {
    println!("the match control flow construct");

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let nickel = Coin::Nickel;
    let nickel_value = value_in_cents(nickel);
    println!("{:?}", nickel_value);
}