#![allow(dead_code)]

mod shared;
mod profiler;

fn foo() {
    profile_scope!("foo");

    std::thread::sleep(std::time::Duration::from_millis(100));

}


fn main() {
    //let obj = profiler::Item::new("main");
    println!("Hello world");
    foo();
    foo();
    profiler::summary();
}
