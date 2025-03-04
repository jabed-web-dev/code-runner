use std::env;
use std::process::Command;
use std::time::Instant;
use std::io::{self, Write};

extern crate winapi;
use winapi::um::consoleapi::GetConsoleMode;
use winapi::um::consoleapi::SetConsoleMode;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::winnt::HANDLE;

const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

fn enable_ansi_colors() -> io::Result<()> {
    unsafe {
        let handle: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }

        let mut mode: u32 = 0;
        if GetConsoleMode(handle, &mut mode) == 0 {
            return Err(io::Error::last_os_error());
        }

        if SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING) == 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_console_title(title: &str) {
    use winapi::um::wincon::SetConsoleTitleW;
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let wide: Vec<u16> = OsStr::new(title).encode_wide().chain(std::iter::once(0)).collect();
    unsafe {
        SetConsoleTitleW(wide.as_ptr());
    }
}

#[cfg(not(target_os = "windows"))]
fn set_console_title(_title: &str) {
    // Do nothing for non-Windows systems
}

fn main() {
    if let Err(err) = enable_ansi_colors() {
        eprintln!("Failed to enable ANSI colors: {}", err);
    }
    
    let args: Vec<String> = env::args().collect();

    let usage = "Usage: --pause? --clear? <program> <script> [arguments...]";
    if args.len() < 3 {
        eprintln!("{}", usage);
        return;
    }

    let mut args_iter = args.iter();
    args_iter.next(); // Skip the first argument (program name)

    let mut pause = false;
    let mut clear = false;

    let arg1 = args_iter.next().unwrap();
    let arg2 = args_iter.next().unwrap();

    if arg1 == "--pause" || arg2 == "--pause" {
        pause = true;
    } 
    if arg1 == "--clear" || arg2 == "--clear" {
        clear = true;
    }

    if (pause && clear && args.len() < 5) || ((pause || clear) && args.len() < 4) || args.len() < 3 {
        eprintln!("{}", usage);
        return;
    }

    let program = if pause && clear {
        args_iter.next().unwrap()
    } else if pause || clear {
        arg2
    } else {
        arg1
    };

    let script = if pause || clear {
        args_iter.next().unwrap()
    } else {
        arg2
    };

    let script_args: Vec<&String> = args_iter.collect();

    let title = format!("[Running] {}", program);
    set_console_title(&title);

    // Clear the terminal before execution
    if clear {
        Command::new("clear")
            .status()
            .expect("Failed to clear terminal");
    }

    let start_time = Instant::now();

    let status = Command::new(program)
        .arg(script)
        .args(&script_args)
        .status()
        .expect(&format!("Failed to execute {}", program));

    let duration = start_time.elapsed();

    let ms = duration.subsec_millis();
    let total_seconds = duration.as_secs();
    let ss = total_seconds % 60;
    let total_minutes = total_seconds / 60;
    let mm = total_minutes % 60;
    let hh = total_minutes / 60;

    let execution_time = if hh > 0 {
        format!("{}h:{}m:{}s.{}ms", hh, mm, ss, ms)
    } else if mm > 0 {
        format!("{}m:{}s.{}ms", mm, ss, ms)
    } else if ss > 0 {
        format!("{}s.{}ms", ss, ms)
    } else {
        format!("{}ms", ms)
    };

    let exit_status = if status.success() {
        "\x1b[0;32m[Done]\x1b[0m"
    } else {
        "\x1b[0;31m[Failed]\x1b[0m"
    };

    println!(
        "\n{} exited with\x1b[35m code={}\x1b[0m in \x1b[34m{}\x1b[0m",
        exit_status,
        status.code().unwrap_or(-1),
        execution_time
    );

    if pause {
        print!("\x1b[90m[Enter]\x1b[0m");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
    }
}
