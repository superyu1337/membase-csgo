use memflow::{Address, VirtualMemory};

use crate::core::structs::CheatCtx;

use super::structs::{vec3::Vec3, player_info::PlayerInfo};

pub struct ClientState {
    pub ptr: Address,
}

impl ClientState {
    pub fn get<'a>(ctx: &'a mut CheatCtx) -> ClientState {
        let offset = ctx.offsets.sigs["dwClientState"];
        let ptr = ctx.process.virt_mem.virt_read_addr32(ctx.engine_module.base + offset)
            .unwrap();

        return ClientState { ptr };
    }

    pub fn get_viewangles<'a>(&self, ctx: &'a mut CheatCtx) -> Vec3 {
        let offset = ctx.offsets.sigs["dwClientState_ViewAngles"];
        ctx.process.virt_mem.virt_read(self.ptr + offset)
            .unwrap()
    }

    pub fn set_viewangles<'a>(&self, ctx: &'a mut CheatCtx, newangles: Vec3) {
        let offset = ctx.offsets.sigs["dwClientState_ViewAngles"];
        ctx.process.virt_mem.virt_write(self.ptr + offset, &newangles)
            .unwrap();
    }

    pub fn get_userinfo_table<'a>(&self, ctx: &'a mut CheatCtx, index: usize) -> PlayerInfo {
        let offset = ctx.offsets.sigs["dwClientState_PlayerInfo"];
        let userinfotable_ptr = ctx.process.virt_mem.virt_read_addr32(self.ptr + offset)
            .unwrap();
        
        let items_ptr = ctx.process.virt_mem.virt_read_addr32(userinfotable_ptr + 0x40)
            .unwrap();

        let items = ctx.process.virt_mem.virt_read_addr32(items_ptr + 0xC)
            .unwrap();

        ctx.process.virt_mem.virt_read(items + 0x28 + ((index - 1) * 0x34))
            .unwrap()
    }
}