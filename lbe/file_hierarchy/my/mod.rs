mod inaccessible;   //inaccessible.rs或inacessible/mod.rs
pub mod nested;     //nested.rs或nested/mod.rs

pub fn call_inaccessible_function() {
    inaccessible::public_function();
}

pub fn function() {
    println!("called my::function()");
}

#[allow(dead_code)]
fn private_function() {
}