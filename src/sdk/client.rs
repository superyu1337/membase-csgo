use memflow::{Address, VirtualMemory};

use crate::core::structs::CheatCtx;

pub fn get_playerresource<'a>(ctx: &'a mut CheatCtx) -> Address {
    let offset = ctx.offsets.sigs["dwPlayerResource"];
    ctx.process.virt_mem.virt_read_addr(ctx.client_module.base + offset,)
        .unwrap()
}

pub fn get_radarbase<'a>(ctx: &'a mut CheatCtx) -> Address {
    let offset = ctx.offsets.sigs["dwRadarBase"];
    let offset = ctx.process.virt_mem.virt_read_addr32(ctx.client_module.base + offset,)
        .unwrap();

    ctx.process.virt_mem.virt_read_addr32(offset + 0x74,)
        .unwrap()
}