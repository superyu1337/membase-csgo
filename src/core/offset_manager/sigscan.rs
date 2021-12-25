use memflow::VirtualMemory;
use memflow_win32::Win32Process;

use crate::core::offset_manager::memory;

use super::config::Signature;

pub type Result<T> = ::std::result::Result<T, String>;

pub fn scan_signature<T: VirtualMemory>(sig: &Signature, process: &mut Win32Process<T>) -> Result<usize> {

    debug!("Begin scan: {}", sig.name);
    debug!("IsWow64: {:?}", !process.proc_info.wow64.is_null());
    debug!("Load module {}", sig.module);
    let mut modules = process.module_list().unwrap().into_iter();
    let module = modules.find(|m| m.name == sig.module).ok_or("Module not found".to_owned())?;
    
    debug!(
        "Module found: {} - Base: {:#X} Size: {:#X}",
        module.name, module.base, module.size
    );

    debug!("fetching module data for {}", module.name);
    let module_data = process.virt_mem.virt_read_raw(module.base, module.size).unwrap();


    debug!("Searching pattern: {}", sig.pattern);
    let mut addr = memory::findpattern::find_pattern(&module_data, &sig.pattern).ok_or("Pattern not found".to_owned())?;
    debug!(
        "Pattern found at: {:#X} (+ base = {:#X})",
        addr,
        addr + module.base.as_usize()
    );

    for (i, o) in sig.offsets.iter().enumerate() {
        debug!("Offset #{}: ptr: {:#X} offset: {:#X}", i, addr, o);

        let pos = (addr as isize).wrapping_add(*o) as usize;
        let data = module_data.get(pos).ok_or_else(|| {
            debug!("WARN OOB - ptr: {:#X} module size: {:#X}", pos, module.size);
            "Offset out of bounds!".to_owned()
        })?;

        let tmp = if !process.proc_info.wow64.is_null() {
            let raw: u32 = unsafe { std::mem::transmute_copy(data) };
            raw as usize
        } else {
            let raw: u64 = unsafe { std::mem::transmute_copy(data) };
            raw as usize
        };

        addr = tmp.wrapping_sub(module.base.as_usize());
        debug!("Offset #{}: raw: {:#X} - base => {:#X}", i, tmp, addr);
    }

    if sig.rip_relative {
        debug!(
            "rip_relative: addr {:#X} + rip_offset {:#X}",
            addr, sig.rip_offset
        );
        addr = (addr as isize).wrapping_add(sig.rip_offset) as usize;
        debug!("rip_relative: addr = {:#X}", addr);

        let rip: u32 = memory::get_raw(&module, module_data, addr, true)
            .ok_or("RIP Relative failed".to_owned())?;

        debug!(
            "rip_relative: addr {:#X} + rip {:#X} + {:#X}",
            addr,
            rip,
            ::std::mem::size_of::<u32>()
        );
        addr = addr.wrapping_add(rip as usize + ::std::mem::size_of::<u32>());
        debug!("rip_relative: addr => {:#X}", addr);
    }

    debug!("Adding extra {:#X}", sig.extra);
    addr = (addr as isize).wrapping_add(sig.extra) as usize;
    if !sig.relative {
        debug!(
            "Not relative, addr {:#X} + base {:#X} => {:#X}",
            addr,
            module.base,
            addr.wrapping_add(module.base.as_usize())
        );
        addr = addr.wrapping_add(module.base.as_usize());
    }

    Ok(addr)
}