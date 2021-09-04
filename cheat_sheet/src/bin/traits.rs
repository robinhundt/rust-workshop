use std::fmt::Display;

trait FancyPrint: Display {
    fn fancy_print(&self) {
        println!("🌈 {} 🌈", self)
    }
}

impl<T: Display> FancyPrint for T {}


fn print_fancy<T: FancyPrint>(s: T) {
    s.fancy_print();
}

fn main() {
    let s = "Hello world!";
    s.fancy_print();
    let x = 5;
    x.fancy_print();
    print_fancy("hello ");
    print_fancy(42);
}