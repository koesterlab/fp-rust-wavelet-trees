// Make me compile and run without error by implementing the Circle struct.

fn main() {
    let radius = 5.0;
    let mut circle = Circle::new(radius);

    // scale the circle with a given integer factor
    circle.scale(2.0);

    // print debug info about the circle
    println!("{:?}", circle);
    // check radius
    assert_eq!(circle.radius, 10.0);
}





















// In order to print debugging info about the Circle, you have to implement
// a so-called trait (this will come later). For now, simply consider the
// information given here: https://doc.rust-lang.org/book/second-edition/ch05-02-example-structs.html#adding-useful-functionality-with-derived-traits
