use std::ffi::CStr;

use crate::clib::libappdir;

pub fn get_app_cache_location() -> Option<String> {
    let result = unsafe {
        let out = libappdir::appdir_get_path(libappdir::PathType_AppCacheLocation);
        let str = CStr::from_ptr(out);
        let str = str.to_owned();
        String::from_utf8(str.to_bytes().to_vec())
    };
    match result {
        Ok(app_cache) => Some(app_cache),
        Err(_) => None,
    }
}
