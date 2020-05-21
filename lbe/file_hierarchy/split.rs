mod my; //找my.rs或my/mod.rs

fn function() {
    println!("called function()");
}

fn main() {
    function();
    my::function();
    my::nested::public_function();
    my::call_inaccessible_function();
}