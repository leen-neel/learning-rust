fn main() {
    let name = "Neel";

    println!("Hello! My name is {}", name);

    let a = 2;
    let b = 2;
    println!("{0} + {1} = {2}", a, b, a + b);

    let mut mutable = "I can be changed!";
    println!("{}", mutable);
    mutable = "I got changed!";
    println!("{}", mutable);

    let (course, category) = ("Rust", "beginner");
    println!("This is a {0} course in {1}", category, course);
}
