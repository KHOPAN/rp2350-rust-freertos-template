#![no_std]

extern crate alloc;

mod freertos;

const TASK_MAIN_NAME: &str = "Main";
const TASK_MAIN_PRIORITY: u32 = 1;
const TASK_MAIN_STACK_SIZE: u32	= 512;

#[unsafe(no_mangle)]
extern "C" fn main() -> core::ffi::c_int {
	freertos::initialize_output();

	if freertos::new_task(task_main, TASK_MAIN_NAME, TASK_MAIN_STACK_SIZE, TASK_MAIN_PRIORITY).is_some() {
		freertos::start_scheduler();
	}

	1
}

fn task_main() {
	freertos::delay_milliseconds(5000);
	println!("Hello, world!");
}
