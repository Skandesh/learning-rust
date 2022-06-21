fn main(){
test_str();

}


fn test_str(){

    //using just str places the object in heap, and it doesn't know the size while compiling
   // let sync:str = "Hey";
//    println!("{}", sync);


//using &str places 'work' in stack, so it's size is predetermined and entries into stack during
//function call, gets deleted after the scope is over.    

let work:&str = "Hey Stack!"; //for some reason this is called 'Borrowed String'
println!("{}", work);



let heap_var:Box<str> = "Hey heap!".into();
println!("Value is in heap - {}",heap_var);


let owned_string:String = String::from("Hey owner");
println!("This is from owned string - {}", owned_string);
}
