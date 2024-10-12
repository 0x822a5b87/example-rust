#![allow(unused_variables, dead_code)]

fn main() {
    if_let_statement()
}

fn if_let_statement() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => {
            println!("The maximum is configured to exclusive is {}", max);
        }

        // To satisfy the match expression, we have to add _ => () after processing
        // just one variant, which is annoying boilerplate code to add.
        None => {  }
    }

    // NOTICE : the middle operator is `=` rather than `==`
    // The syntax if let takes a pattern and an expression separated by an equal sign.
    // It means if `config_max` is `Some` then assigning variable max with value which the variable config_max contains.
    if let Some(max) = config_max {
        println!("The maximum is configured to exclusive is {}", max);
    }
}

fn option_with_placeholder() {
    let dice_roll = 9;

    match dice_roll {
        3 => {
            println!("3")
        }
        7 => {
            println!("7")
        }
        v => {
            println!("v {}", v);
        }
    };
}

fn enum_with_option() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    match y {
        None => {
            println!("None");
        }
        Some(y) => {
            println!("x + y = {}", x + y);
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(x) => Some(x + 1),
        }
    }

    let five = Some(5);
    let seven = plus_one(plus_one(five));
    match seven {
        None => {
            println!("None");
        }
        Some(v) => {
            println!("v = {}", v);
        }
    }
}

fn enum_with_struct() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    struct QuitMessage;
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String);
    struct ChangeColorMessage(i32, i32, i32);

    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => {
                    println!("Quit");
                }
                Message::Move { x, y } => {
                    println!("x: {}, y: {}", x, y);
                }
                Message::Write(msg) => {
                    println!("Write: {}", msg);
                }
                Message::ChangeColor(r, g, b) => {
                    println!("r: {}, g: {}, b: {}", r, g, b);
                }
            }
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn enum_values_with_multiple_field() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));
}

fn enum_values_with_type() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    impl IpAddr {
        fn print(&self) {
            println!("ip addr = {:?}", self);
        }
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    home.print();
}

fn enum_values() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    impl IpAddr {
        fn print(&self) {
            println!("kind = {:?}, address = {}", self.kind, self.address);
        }
    }

    fn route(ip_kind: IpAddrKind) {
        println!("ip kind = {ip_kind:?}")
    }

    // Note that the variants of the enum are namespaced under its identifier,
    // and we use a double colon to separate the two.
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    route(ipv4);
    route(ipv6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("::1"),
    };

    home.print();
    loopback.print();
}