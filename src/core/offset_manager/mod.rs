// Big credits for Hazedumper!

mod memory;
mod config;
mod sigscan;
mod netvar_manager;

use std::collections::BTreeMap;

use memflow::VirtualMemory;
use memflow_win32::Win32Process;

use self::config::Config;
pub struct OffsetManagerOutput {
    pub sigs: BTreeMap<String, usize>,
    pub netvars: BTreeMap<String, usize>
}

pub fn run_scan<T: VirtualMemory>(process: &mut Win32Process<T>) -> OffsetManagerOutput {
    debug!("Loading config: {}", "./config.json");
    let conf = Config::load("./config.json").unwrap_or_default();
    
    let sigs = scan_signatures(&conf, process);
    let netvars = scan_netvars(&sigs, &conf, process).unwrap();

    return OffsetManagerOutput { sigs, netvars }
}

fn scan_signatures<T: VirtualMemory>(conf: &Config, process: &mut Win32Process<T>) -> BTreeMap<String, usize> {
    info!(
        "Starting signature scanning: {} items",
        conf.signatures.len()
    );
    let mut res = BTreeMap::new();

    for sig in &conf.signatures {
        match sigscan::scan_signature(sig, process) {
            Ok(r) => {
                res.insert(sig.name.clone(), r);
                debug!("Found signature: {} => {:#X}", sig.name, r);
            },
            Err(err) => warn!("{} sigscan failed: {}", sig.name, err),
        }
    }

    info!(
        "Finished signature scanning: {}/{} items successful",
        res.len(),
        conf.signatures.len()
    );

    res
}

/// Scan the netvars from the config and return a `Option<Map<i32>>`.
fn scan_netvars<T: VirtualMemory>(sigs: &BTreeMap<String, usize>, conf: &Config, process: &mut Win32Process<T>) -> Option<BTreeMap<String, usize>> {
    info!("Starting netvar scanning: {} items", conf.netvars.len());

    let first = sigs.get("dwGetAllClasses")?;
    let netvars = netvar_manager::netvars::NetvarManager::new(*first, process)?;

    let mut res = BTreeMap::new();
    for netvar in &conf.netvars {
        match netvars.get_offset(&netvar.table, &netvar.prop) {
            Some(o) => {
                res.insert(netvar.name.clone(), o as usize + netvar.offset as usize);
                debug!("Found netvar: {} => {:#X}", netvar.name, o);
            }
            None => warn!("{} netvar failed!", netvar.name),
        };
    }

    info!(
        "Finished netvar scanning: {}/{} items successful",
        res.len(),
        conf.netvars.len()
    );
    Some(res)
}
