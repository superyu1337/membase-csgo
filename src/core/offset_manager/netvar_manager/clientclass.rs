use memflow::Address;
use memflow::VirtualMemory;
use memflow_win32::{Win32ModuleInfo, Win32Process};
use nom::*;

use crate::core::offset_manager::netvar_manager::helpers::parse_string;
use crate::core::offset_manager::netvar_manager::helpers::get;

use super::table::RecvTable;

#[derive(Debug, PartialEq)]
pub struct ClientClass {
    pub id: i32,
    pub name: String,
    pub table: RecvTable,
}

#[derive(Debug)]
pub struct ClientClassIterator<'a, T: VirtualMemory> {
    next_offset: usize,
    module: &'a Win32ModuleInfo,
    module_data: Vec<u8>,
    process: &'a mut Win32Process<T>
}

impl ClientClass {
    // offset_name, offset_table, offset_next, id
    #[rustfmt::skip]
    named!(
        parse_raw<(usize, usize, usize, i32)>,
        do_parse!(
            take!(8)              >>
            offset_name  : le_u32 >>
            offset_table : le_u32 >>
            offset_next  : le_u32 >>
            id           : le_i32 >>
            ((
                offset_name as usize,
                offset_table as usize,
                offset_next as usize,
                id,
            ))
        )
    );

    fn parse<T: VirtualMemory>(base: usize, module: &Win32ModuleInfo, module_data: &[u8], process: &mut Win32Process<T>) -> Option<(ClientClass, usize)> {
        debug!("Starting to parse ClientClass at {:#x}", base);
        let data = process.virt_mem.virt_read_raw(Address::from(base), 0x18).unwrap();
        let (_, (offset_name, offset_table, offset_next, id)) = Self::parse_raw(&data).ok().expect("autism failed");

        let name = parse_string(get(module_data, module.base.as_usize(), offset_name, false)?)
            .ok()?
            .1
            .to_string();

        debug!("Found ClientClass '{}' at {:#x}", name, base);

        let cc = ClientClass {
            id,
            name,
            table: RecvTable::parse(offset_table, module, module_data, process)?,
        };

        Some((cc, offset_next))
    }
}

impl<'a, T: VirtualMemory> ClientClassIterator<'a, T> {
    pub fn new(next_offset: usize, module: &'a Win32ModuleInfo, module_data: Vec<u8>, process: &'a mut Win32Process<T>) -> Self {
        Self {
            next_offset,
            module,
            module_data,
            process
        }
    }
}

impl<'a, T: VirtualMemory> Iterator for ClientClassIterator<'a, T> {
    type Item = ClientClass;

    fn next(&mut self) -> Option<ClientClass> {
        if self.next_offset == 0 {
            return None;
        }

        let (cc, next) = ClientClass::parse(self.next_offset, self.module, self.module_data.as_slice(), self.process)?;

        self.next_offset = next;
        Some(cc)
    }
}
