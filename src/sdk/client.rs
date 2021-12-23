use memflow::{Address, VirtualMemory};

use crate::core::structs::CheatCtx;

pub fn get_playerresource(ctx: &mut CheatCtx) -> Address {
    let offset = ctx.offsets["dwPlayerResource"];
    ctx.process.virt_mem.virt_read_addr32(ctx.client_module.base + offset,)
        .unwrap()
}

pub fn get_radarbase(ctx: &mut CheatCtx) -> Address {
    let offset = ctx.offsets["dwRadarBase"];
    let offset = ctx.process.virt_mem.virt_read_addr32(ctx.client_module.base + offset,)
        .unwrap();

    ctx.process.virt_mem.virt_read_addr32(offset + 0x74,)
        .unwrap()
}