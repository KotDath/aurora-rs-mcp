use std::time::Duration;

use dbus::{Error, blocking::Connection};

const DEST: &str = "ru.omp.deviceinfo";
const PATH: &str = "/ru/omp/deviceinfo/Features";
const IFACE: &str = "ru.omp.deviceinfo.Features";

pub fn ac_device_info_get_device_model() -> Option<String> {
    let conn = match Connection::new_system() {
        Ok(conn) => conn,
        Err(_) => return None,
    };
    let proxy = conn.with_proxy(DEST, PATH, Duration::from_millis(5000));
    let result: Result<(String,), Error> = proxy.method_call(IFACE, "getDeviceModel", ());
    if let Ok((result,)) = result {
        Some(result)
    } else {
        None
    }
}