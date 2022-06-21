fn main(){
    add(3,4);
//    name();
}


//This function takes two arguments and returns an i64 type
fn add(a:i32, b:i32) -> i64{
    println!("The sum of the numbers is {}",a+b);
    return (a+b).into();
}

//Takes nothing and returns nothing
fn name() {
    print!("Rust is fun");
}

