fn main() {
    let i = my_util::add(1, 2);
    println!("(1 + 2) = {i}");

    let hello = my_util::lib2::hello2(&String::from("0"));
    println!("Hello from lib2::hello2 [{}]", hello);
}
