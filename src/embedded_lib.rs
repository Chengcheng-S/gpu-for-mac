use metal::*;
use objc::rc::autoreleasepool;

pub fn embedded() {
    let library_data = include_bytes!("shaders.metallib");

    autoreleasepool(|| {
        let device = Device::system_default().expect("no device found");

        let library = device.new_library_with_data(&library_data[..]).unwrap();
        let kernel = library.get_function("sum", None).unwrap();

        println!("Function name: {}", kernel.name());
        println!("Function type: {:?}", kernel.function_type());
        println!("OK");
    });
}