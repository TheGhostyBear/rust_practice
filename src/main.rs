fn main() {
    println!("Hello, world!");
    println!("This is just a test");
    println!("{subject} {verb} {objects}",
            subject = "This program",
            objects = "a test",
            verb = "performs");
    println!("{} days are for coffee", "All");
    println!("{0}, this is {1}. {1}, this is {0}.", "Bob", "Alice");
    println!("Base 10: {}", 69240); //69420
    println!("Base 2: {:b}", 69420);
    println!("Base 8: {:o}", 69420);
    println!("Base 16: {:x}", 69420);
    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1);
    println!("{number:0<5}", number=1);
    println!("{number:0<width$}", number=1, width=5);
// Fix me from Rust by example  \/
    println!("My name is {0}, {1} {0}.", "Bond", "James");
     // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line (recommented the line after trying it out)
    let pi=3.141592;
    println!("Pi is roughly {pi}.");
    let people = "Rustaceans";
    println!("Hello, {people}!");
}
// Test comment
/* testing nested comment
still nested
nest
//! Generate library docs for the enclosing item.
ed */