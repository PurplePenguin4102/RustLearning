use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("output.txt").expect("create failed");
    file.write_all("出来た!!".as_bytes()).expect("write failed");
    println!("data written to file" );
    println!("出来た!!");
}
