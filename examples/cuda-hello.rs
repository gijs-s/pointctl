#[macro_use]
extern crate rustacuda;

use rustacuda::prelude::*;
use std::error::Error;
use std::ffi::CString;

/// Print hello world from the GPU, note that this code does no print anything.
fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the CUDA API
    rustacuda::init(CudaFlags::empty())?;

    // Get the first device
    let device = Device::get_device(0)?;

    // Create a context associated to this device
    let _context =
        Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;

    // Load the module containing the function we want to call
    let module_data = CString::new(include_str!("../kernels/compiled/hello-world.ptx"))?;
    let module = Module::load_from_string(&module_data)?;

    // Create a stream to submit work to
    let stream = Stream::new(StreamFlags::NON_BLOCKING, None)?;

    // Launching kernels is unsafe since Rust can't enforce safety - think of kernel launches
    // as a foreign-function call. In this case, it is - this kernel is written in CUDA C.
    unsafe {
        let function_name = CString::new("hello_world")?;
        let hello_world = module.get_function(&function_name)?;
        let result = launch!(hello_world<<<1, 1, 0, stream>>>())?;
        result
    }

    // The kernel launch is asynchronous, so we wait for the kernel to finish executing
    stream.synchronize()?;
    Ok(())
}
