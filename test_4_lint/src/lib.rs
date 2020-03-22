pub fn do_something () {
    let x = foo();
    let y = bar();
    println!("x: {} , y: {}", x, y);
}
use std::sync::Arc;
fn foo() -> String {
    "asdf".to_owned()
}

fn bar() -> Arc<String> {
    Arc::new("super bar".to_owned())
}