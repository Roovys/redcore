#[cfg(target_os = "windows")]
mod adlx {
    use std::ffi::{c_char, c_int, c_void};

    type HModule = *mut c_void;
    type FarProc = *const c_void;
    type AdlxResult = c_int;
    type AdlxQueryFullVersionFn = unsafe extern "C" fn(*mut u64) -> AdlxResult;

    const ADLX_OK: AdlxResult = 0;

    #[link(name = "kernel32")]
    extern "system" {
        fn LoadLibraryW(lp_lib_file_name: *const u16) -> HModule;
        fn GetProcAddress(h_module: HModule, lp_proc_name: *const c_char) -> FarProc;
        fn FreeLibrary(h_lib_module: HModule) -> i32;
    }

    pub fn probe_runtime() -> Result<u64, String> {
        let dll_name: Vec<u16> = "amdadlx64.dll\0".encode_utf16().collect();

        unsafe {
            let handle = LoadLibraryW(dll_name.as_ptr());
            if handle.is_null() {
                return Err("Failed to load amdadlx64.dll (AMD Radeon driver not found).".into());
            }

            let symbol = b"ADLXQueryFullVersion\0".as_ptr() as *const c_char;
            let proc = GetProcAddress(handle, symbol);
            if proc.is_null() {
                FreeLibrary(handle);
                return Err("Loaded amdadlx64.dll, but ADLXQueryFullVersion was not found.".into());
            }

            let query_full_version: AdlxQueryFullVersionFn = std::mem::transmute(proc);
            let mut full_version: u64 = 0;
            let res = query_full_version(&mut full_version);
            FreeLibrary(handle);

            if res == ADLX_OK {
                Ok(full_version)
            } else {
                Err(format!("ADLXQueryFullVersion failed with code {res}"))
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn main() {
    println!("RedCore Daemon v0.1.0 — Probing AMD ADLX runtime...");
    match adlx::probe_runtime() {
        Ok(version) => println!("ADLX runtime linked successfully (full version: {version:#x})"),
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("RedCore targets Windows AMD Radeon drivers (amdadlx64.dll).");
}
