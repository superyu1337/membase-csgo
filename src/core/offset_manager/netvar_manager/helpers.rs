use nom::{named, map_res, take_until_and_consume};

named!(
    pub parse_string<&str>,
    map_res!(take_until_and_consume!("\0"), ::std::str::from_utf8)
);

pub fn get(module_data: &[u8], module_base: usize, mut offset: usize, is_relative: bool) -> Option<&[u8]> {
    if !is_relative {
        offset = offset.wrapping_sub(module_base);
    }

    module_data.get(offset..)
}