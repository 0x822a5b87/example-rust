#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    //! # square
    //! Associated functions that aren’t methods are often used for constructors that
    //! will return a new instance of the struct.
    //! These are often called new, but new isn’t a special name and isn’t built into the language.
    pub fn new(width: u32, height: u32) -> Self {
        Self {width, height }
    }
}

// We start an impl (implementation) block for Rectangle.
// Everything within this impl block will be associated with the Rectangle type.
impl Rectangle {
    // 1. methods are defined within the context of a struct (or an enum or a trait object)
    // 2. their first parameter is always self, which represents the instance of the struct the method is being called on.
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}