use memflow::{Address, VirtualMemory};

use crate::core::structs::CheatCtx;

pub struct GlowManager {
    pub ptr: Address
}

impl GlowManager {
    pub fn get<'a>(ctx: &'a mut CheatCtx) -> GlowManager {
        let offset = ctx.offsets.sigs["dwGlowObjectManager"];
        let ptr = ctx.process.virt_mem.virt_read_addr32(ctx.client_module.base + offset)
            .unwrap();

        return GlowManager { ptr };
    }
}