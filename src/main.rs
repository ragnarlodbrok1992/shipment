unsafe extern "C" {
    pub fn add(a: i32, b: i32) -> i32;
}

fn main() {
    println!("Hello from Shipment!");

    // Using library from C++
    unsafe {
        println!("34 + 35 = {}", add(34 ,35));
    }
}
