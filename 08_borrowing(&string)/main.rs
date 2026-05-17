fn print_name(name: &String) {
    println!("{}", name);
}

fn main() {
    let my_name = String::from("Great");

    print_name(&my_name);

    println!("{}", my_name);
}