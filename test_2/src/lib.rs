
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
