pub fn warner_brother() {
    println!("Hello i am warner! not david")
}
// using the parent functions using absolute path
pub fn calling_parent(){
    println!("Calling the function from child module!");
   crate::hollywood::future_se_hai_apun();
}

// calling the parent function using the relative path;
pub fn calling_again(){
    super::future_se_hai_apun();
}
