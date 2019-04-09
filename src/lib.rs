
use winapi::{
	shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
	um::consoleapi,
};

#[no_mangle]
#[allow(unused_variables)]
pub extern "system" fn DllMain(
	dll_module: HINSTANCE,
	call_reason: DWORD,
	reserved: LPVOID)
	-> BOOL {
	const DLL_PROCESS_ATTACH: DWORD = 1;

	match call_reason {
		DLL_PROCESS_ATTACH => init(),
		_ => TRUE
	}
}

fn init() -> BOOL {
	unsafe { consoleapi::AllocConsole() };
	TRUE
}
