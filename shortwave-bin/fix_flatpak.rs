use std::ffi::{c_void, CString};
use std::mem::transmute;
use std::os::raw::{c_char, c_int};

const RTLD_NEXT: *mut c_void = -1i64 as *mut c_void;

extern "C" {
    pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn open64(path: *const c_char, oflag: c_int) -> c_int {
    let original_open64: extern "C" fn(*const c_char, c_int,) -> c_int = {
        let f  = dlsym(RTLD_NEXT, "open64 ".as_ptr() as _);
        transmute(f)
    };

    let p = {
        let p = CString::from_raw(path as _);
        let p2 = p.clone();
        std::mem::forget(p);
        p2
    };
    if p == CString::new("/app/share/shortwave/de.haeckerfelix.Shortwave.Devel.gresource").unwrap()
    {
        original_open64(
            CString::new("/usr/share/shortwave/de.haeckerfelix.Shortwave.Devel.gresource")
                .unwrap()
                .into_raw(),
            oflag,
        )
    } else if p.to_str().unwrap().contains("favicon") {
        // For some reason favicons permission changes sometimes to 000
        // This is probably due to how flatpak sandbox works.
        // This blocks worksaround it.
        let result = original_open64(p.clone().into_raw(), oflag);
        if result == -1 {
            std::process::Command::new("chmod")
                .arg("+rw")
                .arg(p.to_str().unwrap())
                .status()
                .unwrap();
            original_open64(path, oflag)
        } else {
            result
        }
    } else {
        original_open64(path, oflag)
    }
}
