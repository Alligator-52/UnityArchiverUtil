// use std::io;
// //use std::os::windows::io::AsRawHandle;
// use winapi::um::consoleapi::SetConsoleMode;
// use winapi::um::handleapi::INVALID_HANDLE_VALUE;
// use winapi::um::processenv::GetStdHandle;
// use winapi::um::winbase::STD_OUTPUT_HANDLE;
// use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

// pub fn enable_ansi_support() -> io::Result<()> {
//     unsafe {
//         let std_out_handle = GetStdHandle(STD_OUTPUT_HANDLE);
//         if std_out_handle == INVALID_HANDLE_VALUE {
//             return Err(io::Error::last_os_error());
//         }

//         let console_mode: u32 = 0;
//         if SetConsoleMode(std_out_handle, console_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING) == 0 {
//             return Err(io::Error::last_os_error());
//         }
//     }
//     Ok(())
// }