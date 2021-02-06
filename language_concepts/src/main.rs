fn main() {
    println!("Rust Language concepts...");

    variables();    
    data_types();    
    functions();
    control_flow();

    println!("{:?}", String::from("出来た!!"));
}

fn section_header(name: String) {
    let dashes = "-------------------------------------------";
    println!("{}", dashes);
    println!("----{}", name);
    println!("{}", dashes);
}

fn variables() {
    // variables...
    section_header(String::from("Variables"));
    println!("some variables are mutable and some are not...");
    let x = 5;
    let mut y = 4;

    println!("`let x = 5` can't be changed x={}", x);
    println!("`let mut y = 4` can be changed y={}", y);
    y = 12;
    println!("see? y={}", y);

    println!("I can declare a new x though with a diff value");
    let x = 15;

    println!("see? x={}", x);

    const MY_CONST: i32 = 12345;
    println!("consts also exist MY_CONST={} ;; they need an explicit type. This one's i32! Wow! :D", MY_CONST);

    let x = x * 2;
    println!("Apparently shadowing lets you do an operation; x*2={}", x);

    let x = "Wow :D";
    println!("And we can apparently 'change the type' of x using shadowing x={}", x);
    println!("This doesn't work with mutable variables though.");
    println!("");

}

fn data_types() {
    section_header(String::from("Data Types and operations"));

    println!("Oh look, data types!");
    println!("i8, i16, i32, i64!");
    let x: i8 = 16;
    println!("i8 x={}",x);
    let x: i16 = 550;
    println!("i16 x={}",x);
    let x: i32 = 58_000;
    println!("i32 x={}",x);
    let x: i64 = 68_000_000;
    println!("i64 x={}",x);

    println!("Binary, octal and hex literals:");
    let x: u8 = 0b1111_0000;
    println!("0b1111_0000 x={}",x);
    let x: i16 = 0o777;
    println!("0o777 x={}",x);
    let x: i32 = 0xffbca;
    println!("0xffbca x={}",x);

    println!("Operations!");
    println!("3, 0xff: Add {} minus {}", 3 + 0xff, 0xff - 3);
    println!("15, 6: Mul {} Div {}", 15 * 6, 15 / 6);
    println!("floats 3.2, 2.6: {} {}", 3.2 * 2.6, 3.2 / 2.6);
    println!("42, 4: rem {}", 42 % 4);
    
    println!("tuples: {:?}", (1, 1.1, 12));
    let tup = (1, 1.1, "Hello");
    let (_tx, ty, _tz) = tup;
    println!("Splatting!, (x,y,z); y={}", ty);
    println!("Deconstructing!, x.0={}", tup.0);

    println!("arrays: {:?}", [1, 2, 3, 4, 5]);
    let ary = [3; 5];
    println!("init ary=[3;5]; {:?}", ary);
    let arb: [i32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    println!("init arb:[i32; 10]; {:?}", arb);
    println!("");
}

fn functions() {
    section_header(String::from("functions"));
    function_with_params(123);

    let x = 5;
    let y = {
        let z = 4;
        x + z // no semicolon
    };
    println!("y={}",y);
    let six = five();
    println!("six={}", six);
    println!("");
}

fn function_with_params(x: i32) {
    println!("I just got passed {}", x);
}

fn five() -> i32 { 5 }

fn control_flow() {
    //make a decision
    section_header(String::from("Control Flow"));

    let x = 12;

    if x % 4 == 0 {
        println!("x is divisible by 4!");
    } else if x % 3 == 0 {
        println!("This will never be run even though it's true :(");
    }

    if x % 4 == 0 {
        println!("x is divisible by 4!");
    } if x % 3 == 0 {
        println!("x is divisible by 3!");
    }

    //ternary
    let z = if x % 2 == 0 { 2 } else { 1 };
    println!("z = {}", z);

    println!("Basic loop");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    println!("While loop");
    let mut number = 3;

    while number != 0 {
        println!("while iter... {}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //Foreach loop
    println!("foreach loop");
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}