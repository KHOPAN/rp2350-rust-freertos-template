#[macro_export]
macro_rules! print {
	($($arg:tt)*) => {{
		unsafe extern "C" {
			fn write_text(text: *const core::ffi::c_char, length: u32, new_line: u8);
		}

		let text: alloc::string::String = alloc::format!($($arg)*);

		unsafe {
			write_text(text.as_ptr(), text.len() as u32, 0);
		}
	}};
}

#[macro_export]
macro_rules! println {
	() => {{
		unsafe extern "C" {
			fn write_text(text: *const core::ffi::c_char, length: u32, new_line: u8);
		}

		unsafe {
			write_text(core::ptr::null_mut(), 0, 1);
		}
	}};
	($($arg:tt)*) => {{
		unsafe extern "C" {
			fn write_text(text: *const core::ffi::c_char, length: u32, new_line: u8);
		}

		let text: alloc::string::String = alloc::format!($($arg)*);

		unsafe {
			write_text(text.as_ptr(), text.len() as u32, 1);
		}
	}};
}

pub type Task = *mut core::ffi::c_void;

struct FreeRTOSAllocator;

unsafe impl core::alloc::GlobalAlloc for FreeRTOSAllocator {
	unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
		unsafe {
			allocate(layout.size() as u32, layout.align() as u32)
		}
	}

	unsafe fn dealloc(&self, pointer: *mut u8, layout: core::alloc::Layout) {
		unsafe {
			deallocate(pointer, layout.size() as u32, layout.align() as u32);
		}
	}
}

#[global_allocator]
static ALLOCATOR: FreeRTOSAllocator = FreeRTOSAllocator;

unsafe extern "C" {
	fn allocate(size: u32, align: u32) -> *mut u8;
	fn deallocate(pointer: *mut u8, size: u32, align: u32);
	fn rust_panic(text: *const core::ffi::c_char, length: u32) -> !;
	fn vTaskDelete(task_to_delete: *mut core::ffi::c_void);
	fn vTaskStartScheduler();
	fn xTaskCreate(task_code: unsafe extern "C" fn(*mut core::ffi::c_void), name: *const core::ffi::c_char, stack_depth: u32, parameters: *mut core::ffi::c_void, priority: core::ffi::c_ulong, created_task: *mut *mut core::ffi::c_void) -> core::ffi::c_long;
}

pub fn delay_milliseconds(milliseconds: u32) {
	unsafe extern "C" {
		fn delay_milliseconds(milliseconds: u32);
	}

	unsafe {
		delay_milliseconds(milliseconds);
	}
}

pub fn delete_current_task() {
	delete_task(core::ptr::null_mut());
}

pub fn delete_task(task: Task) {
	unsafe {
		vTaskDelete(task);
	}
}

pub fn initialize_output() {
	unsafe extern "C" {
		fn initialize_output();
	}

	unsafe {
		initialize_output();
	}
}

pub fn new_task<T: FnOnce() + Send + 'static>(entrypoint: T, name: &str, stack_depth: u32, priority: u32) -> Option<Task> {
	unsafe extern "C" fn task_entrypoint<T: FnOnce() + Send + 'static>(parameter: *mut core::ffi::c_void) {
		if parameter.is_null() {
			panic!("received null FreeRTOS task parameter");
		}

		unsafe {
			(*alloc::boxed::Box::from_raw(parameter as *mut T))();
		}

		delete_current_task();
	}

	let name: alloc::ffi::CString = alloc::ffi::CString::new(name).expect("unexpected null byte in task name");
	let parameters: *mut core::ffi::c_void = alloc::boxed::Box::into_raw(alloc::boxed::Box::new(entrypoint)) as *mut core::ffi::c_void;
	let mut task: Task = core::ptr::null_mut();

	unsafe {
		if xTaskCreate(task_entrypoint::<T>, name.as_ptr(), stack_depth, parameters, priority, &mut task) != 1 || task == core::ptr::null_mut() {
			drop(alloc::boxed::Box::from_raw(parameters as *mut T));
			return None;
		}
	}

	Some(task)
}

pub fn start_scheduler() {
	unsafe {
		vTaskStartScheduler();
	}
}

#[panic_handler]
fn panic(information: &core::panic::PanicInfo) -> ! {
	let text: alloc::string::String = alloc::string::ToString::to_string(information);

	unsafe {
		rust_panic(text.as_ptr(), text.len() as u32);
	}
}
