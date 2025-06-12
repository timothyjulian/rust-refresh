static APPLICATION: &str = "My Application";

static mut COUNTER: u32 = 0;

unsafe fn increment() {
    COUNTER += 1;
}

#[test]
fn test_statistics() {
    println!("Application: {}", APPLICATION);
}

#[test]
fn test_statics_mut() {
    unsafe {
        increment();
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }
}
