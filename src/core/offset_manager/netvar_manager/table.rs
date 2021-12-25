use memflow::{VirtualMemory, Address};
use memflow_win32::{Win32ModuleInfo, Win32Process};
use nom::*;

use crate::core::offset_manager::netvar_manager::{helpers::{parse_string, get}, prop::RecvPropIterator};

use super::prop::RecvProp;

#[derive(Debug, Clone, PartialEq)]
pub struct RecvTable {
    pub name: String,
    pub props: Vec<RecvProp>,
}

impl RecvTable {
    // offset_name, offset_props, num_props
    #[rustfmt::skip]
    named!(
        parse_raw<(usize, usize, usize)>,
        do_parse!(
            offset_props: le_u32 >>
            num_props: le_u32    >>
            take!(4)             >>
            offset_name: le_u32  >>
            ((
                offset_name as usize,
                offset_props as usize,
                num_props as usize,
            ))
        )
    );

    pub fn parse<'a, T: VirtualMemory>(base: usize, module: &Win32ModuleInfo, module_data: &[u8], process: &'a mut Win32Process<T>) -> Option<Self> {
        trace!("Starting to parse RecvTable at {:#x}", base);
        if base == 0 {
            return None;
        }

        let data = process.virt_mem.virt_read_raw(Address::from(base), 0x10).unwrap();

        let (_, (offset_name, offset_props, num_props)) = Self::parse_raw(&data).ok()?;
        let name = parse_string(get(&module_data, module.base.as_usize(), offset_name, false)?)
            .ok()?
            .1
            .to_string();

        trace!("Found RecvTable '{}' at {:#x}", name, base);

        Some(Self {
            name,
            props: RecvPropIterator::new(offset_props, num_props, module, module_data, process).collect::<Vec<_>>(),
        })
    }

    pub fn get_offset(&self, name: &str) -> Option<i32> {
        for prop in &self.props {
            if let Some(o) = prop.get_offset(name) {
                return Some(o);
            }
        }
        None
    }
}