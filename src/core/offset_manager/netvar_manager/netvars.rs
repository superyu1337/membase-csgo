use std::collections::BTreeMap;

use memflow::VirtualMemory;
use memflow_win32::Win32Process;

use crate::core::offset_manager::netvar_manager::clientclass::ClientClassIterator;

use super::table::RecvTable;


#[derive(Debug, Clone, PartialEq)]
pub struct NetvarManager {
    tables: BTreeMap<String, RecvTable>,
}

impl NetvarManager {
    pub fn new<'a, T: VirtualMemory>(first: usize,process: &'a mut Win32Process<T>) -> Option<Self> {
        let modules = process.module_list().unwrap();
        let module = modules
            .iter()
            .find(|m| m.name == "client.dll")
            .unwrap();

        debug!("First ClientClass at {:#X}", first);

        debug!("Fetching module data!");
        let module_data = process.virt_mem.virt_read_raw(module.base, module.size).unwrap();

        let classes = ClientClassIterator::new(
            first + module.base.as_usize(), 
            module, 
            module_data.clone(),
            process
        );

        let tables = classes
            .map(|c| (c.table.name.clone(), c.table))
            .collect::<BTreeMap<_, _>>();

        debug!("Added {} parent RecvTables!", tables.len());
        Some(NetvarManager { tables })
    }

    pub fn get_offset(&self, table_name: &str, netvar_name: &str) -> Option<i32> {
        self.tables.get(table_name)?.get_offset(netvar_name)
    }
}