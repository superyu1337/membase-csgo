pub mod offset_manager;
pub mod structs;

use memflow::{ConnectorInventory, ConnectorArgs};
use memflow_win32::{Win32Process, Kernel, Error};

pub unsafe fn setup<'a>(config: structs::Config) -> Result<structs::CheatCtx<'a>, Error> {
    let offsets = self::offset_manager::get_offsets();

    info!("Parsed offsets");

    let inventory = ConnectorInventory::scan();
    let connector = inventory.create_connector(
        "qemu_procfs", &ConnectorArgs::default()
    )?;

    info!("Created connector");

    let mut kernel = Kernel::builder(connector)
        .build_default_caches()
        .build()?;

    info!("Created kernel with version \"{}\" - addr: {}", kernel.kernel_info.kernel_winver, kernel.kernel_info.kernel_base);

    let proc_info = kernel.process_info("csgo.exe")?;
    let mut process = Win32Process::with_kernel(kernel, proc_info);

    info!("Found csgo process - {} - addr: {}", process.proc_info.pid, process.proc_info.address);

    let mut modules = process.module_list()?.into_iter();

    let client_module = modules.clone().find(|m| m.name == "client.dll")
        .ok_or(Error::Other("Could not find the client module!")).unwrap();

    info!("Found client module - addr: {}", client_module.base);

    let engine_module = modules.find(|m| m.name == "engine.dll")
        .ok_or(Error::Other("Could not find the engine module!")).unwrap();

    info!("Found engine module - addr: {}", engine_module.base);

    return Ok(structs::CheatCtx {
        process,
        engine_module,
        client_module,
        offsets,
        config,
    })
}