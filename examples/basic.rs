use rust_profiler::*;

// To execture with profiling:
// cargo run --examples basic --feature profiler

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
        {
            profile_scope!("foo-inner-inner-inner");
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

    }
    {
        profile_scope!("foo-double");
        std::thread::sleep(std::time::Duration::from_millis(100));
        {
            profile_scope!("foo-double-inner");
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
    foo();
    bar();
    foo();

    profiler_summary!();
}
