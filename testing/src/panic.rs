fn main(){
check();

}


fn check(){
    println!("See if this is printing");
    panic!("This shouldn't return after this"); // for some reason 'panic' is called as diverging function
    println!("I shoudln't see whatever is written here");
}
