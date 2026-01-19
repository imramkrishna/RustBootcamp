pub fn print_generic() {
    println!("{:?}", compare('a', 'b'));
    println!("{}", compare(10, 20));
}
//Here this generic prevented code duplication , we donot need to write different functions for different data types. Generics fixed it with a single funtion.

fn compare<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
