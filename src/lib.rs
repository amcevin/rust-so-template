use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn loop_once() -> i32 {
    println!("this is loop_once from rust !!!");
    return 0;
}

#[no_mangle]
pub extern "C" fn trigger_once() -> i32 {
    println!("this is trigger_once from rust !!!");
    return 0;
}

#[no_mangle]
pub extern "C" fn add(a: *const c_char, b: *const c_char) -> i32 {
    // 将参数转换为 CStr
    let a_cstr = unsafe { CStr::from_ptr(a) };
    let b_cstr = unsafe { CStr::from_ptr(b) };

    // 将 CStr 转换为 Rust 字符串
    let a_str = a_cstr.to_str().expect("Invalid UTF-8 in 'a'");
    let b_str = b_cstr.to_str().expect("Invalid UTF-8 in 'b'");

    // 执行你的操作
    let result = your_add_function(a_str, b_str); // 替换为你的实际操作函数

    // 返回结果
    result
}

fn your_add_function(a: &str, b: &str) -> i32 {
    println!("a: {}, b: {}", a, b);

    // 在这里执行你的操作，这里只是一个示例
    let a_int = a.parse::<i32>().unwrap_or(0);
    let b_int = b.parse::<i32>().unwrap_or(0);
    a_int + b_int
}
