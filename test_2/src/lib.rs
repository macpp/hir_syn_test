#![feature(register_attr)]
#![register_attr(my_attr)]
#[derive(Debug)]
pub enum Foo {
    Yes,
    No
}

pub fn foo(f: Foo) {
    match f{
        Foo::Yes => println!("a"),
        Foo::No => println!("b"),        
    }
}

pub fn check(x: i32) {
    if (x == 0) {
        println!(" x == 0")
    }
}

pub fn print(x: Vec<String> )
{
    for y in x.into_iter() {
        println!("vec element: {}",y);
    }
}

pub fn multiple_match(x: i32) {
    match x {
        #[my_attr] 0 /*| /*#[my_attr] */12*/ => println!("ok"), // second attribute will not work
        _ => println!("not ok")
    }
}