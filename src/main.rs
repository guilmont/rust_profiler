#![allow(dead_code)]

mod profiler;

/// TODO: We need to handle the case in which we have multiple scopes at the same level in the same function scope
fn foo() {
    profile_scope!("foo");
    std::thread::sleep(std::time::Duration::from_millis(100));
    {
        profile_scope!("foo-inner");
        std::thread::sleep(std::time::Duration::from_millis(100));
        {
            profile_scope!("foo-inner-inner");
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}

fn bar() {
    profile_scope!("bar");
    std::thread::sleep(std::time::Duration::from_millis(100));
    {
        profile_scope!("bar-inner");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn main() {
    println!("Hello world");
    foo();
    bar();
    foo();

    profiler_summary!();
}
