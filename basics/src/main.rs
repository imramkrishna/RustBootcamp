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
fn check_string(string:String){
    println!("The string is {}",string);
}
fn check_string2(str:&String){
    println!("The string is {}",str);
    //str.push_str("Extra string"); --> as str is not passed as mutable string reference, this will throw error.
}
fn check_string3(str:&mut String){
    str.push_str(" pushed string"); // str reference which is being passed must be a mutable reference.
}
struct User{
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool,
}
fn print_stucture(u:&mut User){
    println!("The username is : {}",u.username);
    println!("The email is : {}",u.email);
    println!("The sign in count is : {}",u.sign_in_count);
    println!("The active status is : {}",u.active);
}

fn main() {
    println!("Hello, world!");
    check_integer_module();
    check_maths_module();
    check_floats_module();
    let s1=String::from("I am a string.");
    let s2:String=String::from("I am a string2.");
    let mut s3=String::from("I am a string3.");
    check_string(s1);
    // println!("{}",s1); --> you cannot use s1 again bcz its ownership is transferred to string variable. to avoid pointer issues, you cannot create a direct copy of a string, or same location pointer for two different strings. You can use s1.clone() and pass it as an argument to use s1 furhter in the code but this creates a copy of that same string in the heap memory which is not a good approach.Further you can also make the check_string() func return the string and store it in some new variable or mut previous variable so that you donot lose the variable.

    // the concept of reference and mutable reference can be used to avoid this error.But you cannot have more than one mutable reference of a same variable, however you can can multiple immutable references.
    check_string2(&s2);
    println!("{}",s2); // --> s2 won't throw any error.But you cannot mutate this string. In order to mutate it, you need to pass &mut s2 in the check string;\
    check_string3(&mut s3);
    println!("{}",s3);
    println!("\n\n\n\nThe structure data is : ");
    //Working with structure
    let mut user=User{
        username:String::from("rkrishna"),
        email:String::from("rkrishna@gmail.com"),
        sign_in_count:1,
        active:true,
    };
    print_stucture(&mut user);
    println!("Bye, World!");
}
