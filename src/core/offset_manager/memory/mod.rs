pub mod findpattern;
use memflow_win32::Win32ModuleInfo;

/// o: Offset
/// is_relative: Base has already been subtracted.
pub fn get_raw<T: Copy>(module: &Win32ModuleInfo, data: Vec<u8>, mut o: usize, is_relative: bool) -> Option<T> {
    if !is_relative {
        o -= module.base.as_usize();
    }
    if o + std::mem::size_of::<T>() >= data.len() {
        return None;
    }
    let ptr =   data.get(o)?;
    let raw: T = unsafe { std::mem::transmute_copy(ptr) };
    Some(raw)
}