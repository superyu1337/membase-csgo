use memflow::{VirtualMemory, Address};
use memflow_win32::{Win32ModuleInfo, Win32Process};
use nom::*;

use crate::core::offset_manager::netvar_manager::helpers::{parse_string, get};

use super::table::RecvTable;

#[derive(Debug, Clone, PartialEq)]
pub struct RecvProp {
    pub name: String,
    pub offset: i32,
    pub table: Option<RecvTable>,
}

#[derive(Debug)]
pub struct RecvPropIterator<'a, T: VirtualMemory> {
    base: usize,
    current: usize,
    max: usize,
    module: &'a Win32ModuleInfo,
    module_data: &'a [u8],
    process: &'a mut Win32Process<T>
}

impl RecvProp {
    #[rustfmt::skip]
    named!(
        parse_raw<(usize, usize, i32)>,
        do_parse!(
            offset_name  : le_u32 >>
            take!(0x24)           >>
            offset_table : le_u32 >>
            value        : le_i32 >>
            ((
                offset_name as usize,
                offset_table as usize,
                value,
            ))
        )
    );

    fn parse<'a, T: VirtualMemory>(base: usize, module: &Win32ModuleInfo, module_data: &[u8], process: &'a mut Win32Process<T>) -> Option<RecvProp> {
        trace!("Starting to parse RecvProp at {:#x}", base);
        let data = process.virt_mem.virt_read_raw(Address::from(base), 0x30).unwrap();
        let (_, (offset_name, offset_table, value)) = RecvProp::parse_raw(&data).ok()?;

        let name = parse_string(get(&module_data, module.base.as_usize(), offset_name, false)?)
            .ok()?
            .1
            .to_string();

            trace!(
                "Found RecvProp '{}' at {:#x}, value {:#x} childtable {:#X}",
                name,
                base,
                value,
                offset_table
            );

        let table = match offset_table {
            0 => None,
            _ => RecvTable::parse(offset_table, module, module_data, process),
        };

        Some(Self {
            name,
            offset: value,
            table,
        })
    }

    pub fn get_offset(&self, name: &str) -> Option<i32> {
        if self.name == name {
            return Some(self.offset);
        }

        match self.table {
            Some(ref table) => match table.get_offset(name) {
                Some(o) => Some(o + self.offset),
                _ => None,
            },
            _ => None,
        }
    }
}

impl<'a, T: VirtualMemory> RecvPropIterator<'a, T> {
    pub fn new(base: usize, max: usize, module: &'a Win32ModuleInfo, module_data: &'a [u8], process: &'a mut Win32Process<T>) -> Self {
        Self {
            base,
            current: 0,
            max,
            module,
            module_data,
            process
        }
    }
}

impl<'a, T: VirtualMemory> Iterator for RecvPropIterator<'a, T> {
    type Item = RecvProp;

    fn next(&mut self) -> Option<RecvProp> {
        if self.current >= self.max {
            return None;
        }

        let prop = RecvProp::parse(self.base + self.current * 0x3C, self.module, self.module_data.clone(), self.process)?;
        self.current += 1;

        Some(prop)
    }
}
