// we declare a "garden" module in our crate root file, the compiler will look for the module's code
// in these places:
//

fn main() {
    let plant = garden::vegetables::Asparagus {};
    println!("I'm growing {plant:?}!")
}

mod garden {
    pub mod vegetables{
        #[derive(Debug)]
        pub struct Asparagus {}
    }
}