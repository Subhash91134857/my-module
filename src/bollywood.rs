// There is no need to declare the module using the "mod" keyword in bollywood.rs, beacuse it's automatically inferred.
pub mod bollsubmod;
pub fn tseries() {
    println!("I am your external!");
    bollsubmod::boll_sub_mod();
}
