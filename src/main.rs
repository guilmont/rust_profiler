#![allow(dead_code)]

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
    println!("Hello world");
    foo();
    foo();

    profiler_summary!();
}
