pub mod offset_manager;
pub mod structs;

use memflow::{ConnectorInventory, ConnectorArgs};
use memflow_win32::{Win32Process, Kernel, Error};

pub unsafe fn setup(config: structs::Config) -> Result<structs::CheatCtx, Error> {
    let offsets = self::offset_manager::get_offsets();

    debug!("Parsed offsets");

    let inventory = ConnectorInventory::scan();
    let connector = inventory.create_connector(
        "qemu_procfs", &ConnectorArgs::default()
    )?;

    debug!("Created connector");

    let mut kernel = Kernel::builder(connector)
        .build()?;

    debug!("Created kernel with version \"{}\" - addr: {}", kernel.kernel_info.kernel_winver, kernel.kernel_info.kernel_base);

    let proc_info = kernel.process_info("csgo.exe")?;
    let mut process = Win32Process::with_kernel(kernel, proc_info);

    debug!("Found csgo process - {} - addr: {}", process.proc_info.pid, process.proc_info.address);

    let mut modules = process.module_list()?.into_iter();

    let client_module = modules.clone().find(|m| m.name == "client.dll")
        .ok_or(Error::Other("Could not find the client module!")).unwrap();

    debug!("Found client module - addr: {}", client_module.base);

    let engine_module = modules.find(|m| m.name == "engine.dll")
        .ok_or(Error::Other("Could not find the engine module!")).unwrap();

    debug!("Found engine module - addr: {}", engine_module.base);

    return Ok(structs::CheatCtx {
        process: process,
        engine_module,
        client_module,
        offsets,
        config,
    })
}