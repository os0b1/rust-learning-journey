fn add_text(name: &mut String) {
    name.push_str(" Dev");
}

fn main() {
    let mut my_name = String::from("Great");

    add_text(&mut my_name);

    println!("{}", my_name);
}