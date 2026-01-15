mod integer;
mod maths;
mod floats;
use maths::*; //  This * import signifies all the fns to be imported can be used directly wihout using the namespace infront of it

fn check_integer_module(){
    println!("The sum of Signed Integers is {}",integer::sum_signed_integer(-5, 7));
    println!("The sum of Unsigned Integers is {}",integer::sum_unsigned_integer(5, 7));
}
fn check_maths_module(){
    println!("The sum of 5 and 7 is {}",add(5,7));
    println!("The difference of 5 and 7 is {}",subtract(5,7));
    println!("The product of 5 and 7 is {}",multiply(5,7));
    println!("The quotient of 5 and 7 is {}",divide(5,7));
}
fn check_floats_module(){
    println!("The sum of 5.1 and 7.0 is {}",floats::float_sum(5.1,7.0));
    println!("The difference of 5.0 and 7.2 is {}",floats::float_sub(5.0,7.2));
}
fn main() {
    println!("Hello, world!");
    check_integer_module();
    check_maths_module();
    check_floats_module();
    println!("Bye, World!");
}
