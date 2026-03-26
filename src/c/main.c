#include <FreeRTOS.h>
#include <pico/stdlib.h>
#include <stdio.h>
#include <string.h>
#include <task.h>

uint8_t* allocate(const uint32_t size, const uint32_t align) {
	if(size && align <= portBYTE_ALIGNMENT) {
		return pvPortMalloc(size);
	}

	const uint64_t total = (uint64_t) size + align + 3;

	if(total > UINT32_MAX) {
		return 0;
	}

	void* const buffer = pvPortMalloc((uint32_t) total);

	if(!buffer) {
		return 0;
	}

	uint8_t* const pointer = (uint8_t*) ((((uint32_t) buffer) + align - 1) & ~(align - 1));
	memcpy(pointer + size, &buffer, 4);
	return pointer;
}

[[noreturn]]
void assertion_failed(const char* const file_name, const int line_number) {
	printf("Assertion failed in %s at line %d\n", file_name, line_number);
	while(1);
}

void deallocate(uint8_t* const pointer, const uint32_t size, const uint32_t align) {
	if(size && align <= portBYTE_ALIGNMENT) {
		vPortFree(pointer);
		return;
	}

	void* buffer;
	memcpy(&buffer, pointer + size, 4);
	vPortFree(buffer);
}

void delay_milliseconds(const uint32_t milliseconds) {
	vTaskDelay(pdMS_TO_TICKS(milliseconds));
}

void initialize_output() {
	stdio_init_all();
	setvbuf(stdout, 0, _IONBF, 0);
}

[[noreturn]]
void rust_panic(const char* const text, const uint32_t length) {
	printf("Panic: ");
	fwrite(text, sizeof(char), length, stdout);
	printf("\n");
	while(1);
}

[[noreturn]]
void vApplicationMallocFailedHook() {
	printf("Memory allocation failed\n");
	while(1);
}

[[noreturn]]
void vApplicationStackOverflowHook(const TaskHandle_t task, char* const task_name) {
	printf("Stack overflow in task: %s\n", task_name);
	while(1);
}

void write_text(const char* const text, const uint32_t length, const uint8_t new_line) {
	if(text && length) {
		fwrite(text, sizeof(char), length, stdout);
	}

	if(new_line) {
		fwrite("\n", sizeof(char), 1, stdout);
	}
}
