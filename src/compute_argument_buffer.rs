use metal::*;
use objc::rc::autoreleasepool;
use std::mem;

static LIBRARY_SRC: &str = include_str!("../build_miden/compute_argument_buffer/compute-argument-buffer.metal");

pub fn buffers() {
    autoreleasepool(|| {
        println!("start buffers =======> ");
        let device = Device::system_default().expect("no device found");
        let command_queue = device.new_command_queue();

        let data = [
            1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30,
        ];

        let buffer = device.new_buffer_with_data(
            unsafe { mem::transmute(data.as_ptr()) },
            (data.len() * mem::size_of::<u32>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        );

        let sum = {
            let data = [0u32];
            device.new_buffer_with_data(
                unsafe { mem::transmute(data.as_ptr()) },
                (data.len() * mem::size_of::<u32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache,
            )
        };

        let command_buffer = command_queue.new_command_buffer();
        let encoder = command_buffer.new_compute_command_encoder();

        let library = device
            .new_library_with_source(LIBRARY_SRC, &CompileOptions::new())
            .unwrap();
        let kernel = library.get_function("sum", None).unwrap();

        let argument_encoder = kernel.new_argument_encoder(0);
        let arg_buffer = device.new_buffer(
            argument_encoder.encoded_length(),
            MTLResourceOptions::empty(),
        );
        argument_encoder.set_argument_buffer(&arg_buffer, 0);
        argument_encoder.set_buffer(0, &buffer, 0);
        argument_encoder.set_buffer(1, &sum, 0);

        let pipeline_state_descriptor = ComputePipelineDescriptor::new();
        pipeline_state_descriptor.set_compute_function(Some(&kernel));

        let pipeline_state = device
            .new_compute_pipeline_state_with_function(
                pipeline_state_descriptor.compute_function().unwrap(),
            )
            .unwrap();

        encoder.set_compute_pipeline_state(&pipeline_state);
        encoder.set_buffer(0, Some(&arg_buffer), 0);

        encoder.use_resource(&buffer, MTLResourceUsage::Read);
        encoder.use_resource(&sum, MTLResourceUsage::Write);

        let width = 16;

        let thread_group_count = MTLSize {
            width,
            height: 1,
            depth: 1,
        };

        let thread_group_size = MTLSize {
            width: (data.len() as u64 + width) / width,
            height: 1,
            depth: 1,
        };

        encoder.dispatch_thread_groups(thread_group_count, thread_group_size);
        encoder.end_encoding();
        command_buffer.commit();
        command_buffer.wait_until_completed();

        let ptr = sum.contents() as *mut u32;
        println!("sum = {}", unsafe { *ptr });
        
        unsafe {
            assert_eq!(465, *ptr);
        }
    });
}

pub fn bind(){
    autoreleasepool(||{
        println!("start binding =======> ");
        let device = Device::system_default().expect("no device found");

        let buffer = device.new_buffer(4, MTLResourceOptions::empty());
        let sampler = {
            let descriptor = SamplerDescriptor::new();
            device.new_sampler(&descriptor)
        };

        let queue = device.new_command_queue();
        let cmd_buf = queue.new_command_buffer();

        let encoder = cmd_buf.new_compute_command_encoder();

        encoder.set_buffers(2, &[Some(&buffer), None], &[4, 0]);
        encoder.set_sampler_states(1, &[Some(&sampler), None]);

        encoder.end_encoding();
        cmd_buf.commit();

        println!("Everything is bound");
    });
}


pub fn caps(){
    autoreleasepool(||{
        println!("start caps =======> ");
        let device = Device::system_default().expect("no device found");

        #[cfg(feature = "private")]
        {
            println!("Vendor: {:?}", unsafe { device.vendor() });
            println!("Family: {:?}", unsafe { device.family_name() });
        }

        println!(
            "Max threads per threadgroup: {:?}",
            device.max_threads_per_threadgroup()
        );

        #[cfg(target_os = "macos")]
        {
            println!("Integrated GPU: {:?}", device.is_low_power());
            println!("Headless: {:?}", device.is_headless());
            println!("D24S8: {:?}", device.d24_s8_supported());
            println!("Supports dynamic libraries: {:?}", device.supports_dynamic_libraries());
        }
        println!("maxBufferLength: {} Mb", device.max_buffer_length() >> 20);
        println!(
            "Indirect argument buffer: {:?}",
            device.argument_buffers_support()
        );
        
    });
}