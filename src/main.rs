#![allow(dead_code)]

mod shared;
mod profiler;

fn foo() {
    profile_scope!("foo");
    std::thread::sleep(std::time::Duration::from_millis(100));

    {
        profile_scope!("ar");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

}


fn main() {
    {
        profile_scope!("main");
        println!("Hello world");
        foo();
        foo();
    }
    profiler::summary();
}
