extern crate nalgebra_glm as glm;
use std::f64::consts::PI;
use std::ffi::CString;
use std::{mem, os::raw::c_void};

pub unsafe fn get_gl_string(name: gl::types::GLenum) -> String {
    std::ffi::CStr::from_ptr(gl::GetString(name) as *mut i8)
        .to_string_lossy()
        .to_string()
}

// Debug callback to panic upon enountering any OpenGL error
pub extern "system" fn debug_callback(
    source: u32,
    e_type: u32,
    id: u32,
    severity: u32,
    _length: i32,
    msg: *const i8,
    _data: *mut std::ffi::c_void,
) {
    if e_type != gl::DEBUG_TYPE_ERROR {
        return;
    }
    if severity == gl::DEBUG_SEVERITY_HIGH || severity == gl::DEBUG_SEVERITY_MEDIUM || severity == gl::DEBUG_SEVERITY_LOW {
        let severity_string = match severity {
            gl::DEBUG_SEVERITY_HIGH => "high",
            gl::DEBUG_SEVERITY_MEDIUM => "medium",
            gl::DEBUG_SEVERITY_LOW => "low",
            _ => "unknown",
        };
        unsafe {
            let string = CString::from_raw(msg as *mut i8);
            let error_message = String::from_utf8_lossy(string.as_bytes()).to_string();
            panic!(
                "{}: Error of severity {} raised from {}: {}\n",
                id, severity_string, source, error_message
            );
        }
    }
}

/** Get size of array in number of bytes */
pub fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

/** Get the OpenGL-compatible pointer to an arbitrary array of numbers */
pub fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

/** Get the size of the given type in bytes*/
pub fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

/** Get an offset in bytes for n units of type T */
pub fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

/*


For animating the 5 helictopers in task 6


*/

pub struct Heading {
    pub x: f32,
    pub z: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

pub fn simple_heading_animation(time: f32) -> Heading {
    let t = time as f64;
    let step = 0.05f64;
    let path_size = 15f64;
    let circuit_speed = 0.8f64;

    let xpos = path_size * (2.0 * (t + 0.0) * circuit_speed).sin();
    let xpos_next = path_size * (2.0 * (t + step) * circuit_speed).sin();
    let zpos = 3.0 * path_size * ((t + 0.0) * circuit_speed).cos();
    let zpos_next = 3.0 * path_size * ((t + step) * circuit_speed).cos();

    let delta_pos = glm::vec2(xpos_next - xpos, zpos_next - zpos);

    let roll = (t * circuit_speed).cos() * 0.5;
    let pitch = -0.175 * glm::length(&delta_pos);
    let yaw = PI + delta_pos.x.atan2(delta_pos.y);

    Heading {
        x: xpos as f32,
        z: zpos as f32,
        roll: roll as f32,
        pitch: pitch as f32,
        yaw: yaw as f32,
    }
}
