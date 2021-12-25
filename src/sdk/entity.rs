use memflow::{Address, VirtualMemory};
use crate::core::structs::CheatCtx;

use super::structs::{matrix3x4::Matrix3x4, vec3::Vec3, radar_player::RadarPlayer};

pub struct Entity {
    pub ptr: Address,
    pub index: usize,
}

impl Entity {
    pub fn from_index<'a>(ctx: &'a mut CheatCtx, index: usize) -> Entity {
        let offset = ctx.offsets.sigs["dwEntityList"];
        let ptr = ctx.process.virt_mem.virt_read_addr32(ctx.client_module.base + offset + (index * 0x10))
            .expect(&format!("Read entity id {}", index));

        return Entity { ptr, index};
    }

    pub fn get_local<'a>(ctx: &'a mut CheatCtx) -> Entity {
        let offset = ctx.offsets.sigs["dwLocalPlayer"];
        let ptr = ctx.process.virt_mem.virt_read_addr32(ctx.client_module.base + offset)
            .unwrap();

        let index: i32 = ctx.process.virt_mem.virt_read(ptr + 0x64)
            .unwrap();

        return Entity { ptr, index: index as usize };
    }

    pub fn get_index<'a>(&self, ctx: &'a mut CheatCtx) -> i32 {
        ctx.process.virt_mem.virt_read(self.ptr + 0x64)
            .unwrap()
    }

    pub fn get_health<'a>(&self, ctx: &'a mut CheatCtx) -> i32 {
        let offset = ctx.offsets.netvars["m_iHealth"];
        ctx.process.virt_mem.virt_read(self.ptr + offset)
            .unwrap()
    }

    pub fn get_team<'a>(&self, ctx: &'a mut CheatCtx) -> i32 {
        let offset = ctx.offsets.netvars["m_iTeamNum"];
        ctx.process.virt_mem.virt_read(self.ptr + offset)
            .unwrap()
    }

    pub fn get_dormant<'a>(&self, ctx: &'a mut CheatCtx) -> bool {
        let offset = ctx.offsets.netvars["m_bDormant"];
        let data: u8 = ctx.process.virt_mem.virt_read(self.ptr + offset)
            .unwrap();

        return data != 0;
    }

    pub fn get_glowindex<'a>(&self, ctx: &'a mut CheatCtx) -> usize {
        let offset = ctx.offsets.netvars["m_iGlowIndex"];
        let data: i32 = ctx.process.virt_mem.virt_read(self.ptr + offset)
            .unwrap();

        return data as usize;
    }

    pub fn get_pos<'a>(&self, ctx: &'a mut CheatCtx) -> Vec3 {
        let offset = ctx.offsets.netvars["m_vecOrigin"];
        ctx.process.virt_mem.virt_read(self.ptr + offset)
            .unwrap()
    }

    pub fn get_viewoffset<'a>(&self, ctx: &'a mut CheatCtx) -> Vec3 {
        let offset = ctx.offsets.netvars["m_vecViewOffset"];
        ctx.process.virt_mem.virt_read(self.ptr + offset)
            .unwrap()
    }

    pub fn get_aimpunch<'a>(&self, ctx: &'a mut CheatCtx) -> Vec3 {
        let offset = ctx.offsets.netvars["m_aimPunchAngle"];
        ctx.process.virt_mem.virt_read(self.ptr + offset)
            .unwrap()
    }

    pub fn get_spotted<'a>(&self, ctx: &'a mut CheatCtx) -> bool {
        let offset = ctx.offsets.netvars["m_bSpotted"];
        let data: u8 = ctx.process.virt_mem.virt_read(self.ptr + offset)
            .unwrap();

        return data != 0;
    }

    pub fn set_spotted<'a>(&self, ctx: &'a mut CheatCtx, value: bool) {
        let offset = ctx.offsets.netvars["m_bSpotted"];
        ctx.process.virt_mem.virt_write(self.ptr + offset, &(value as u8))
            .unwrap();
    }

    pub fn get_spotted_mask<'a>(&self, ctx: &'a mut CheatCtx) -> i32 {
        let offset = ctx.offsets.netvars["m_bSpottedByMask"];
        ctx.process.virt_mem.virt_read(self.ptr + offset)
            .unwrap()
    }

    pub fn get_bonematrix_ptr<'a>(&self, ctx: &'a mut CheatCtx) -> Address {
        let offset = ctx.offsets.netvars["m_dwBoneMatrix"];
        ctx.process.virt_mem.virt_read_addr32(self.ptr + offset)
            .unwrap()
    }

    pub fn get_bonepos<'a>(&self, ctx: &'a mut CheatCtx, bone_id: usize) -> Vec3 {
        let bonematrix_ptr = self.get_bonematrix_ptr(ctx);
        let bonematrix: Matrix3x4 = ctx.process.virt_mem.virt_read(bonematrix_ptr + (bone_id * 0x30))
            .unwrap();

        Vec3::new(bonematrix.row0[3], bonematrix.row1[3], bonematrix.row2[3])
    }

    pub fn get_comp_rank<'a>(&self, ctx: &'a mut CheatCtx, player_resources: Address) -> i32 {
        let offset = ctx.offsets.netvars["m_iCompetitiveRanking"];
        ctx.process.virt_mem.virt_read(player_resources + offset + (self.index * 0x4))
            .unwrap()
    }

    pub fn get_comp_wins<'a>(&self, ctx: &'a mut CheatCtx, player_resources: Address) -> i32 {
        let offset = ctx.offsets.netvars["m_iCompetitiveWins"];
        ctx.process.virt_mem.virt_read(player_resources + offset + (self.index * 0x4))
            .unwrap()
    }
    
    pub fn get_radarplayer<'a>(&self, ctx: &'a mut CheatCtx, radar_base: Address) -> RadarPlayer {
        ctx.process.virt_mem.virt_read(radar_base + ( 0x174 * (self.index + 1)) - 0x3C)
            .unwrap()
    }
}

pub const RANKS: [&str; 19] = [
    "Unranked",
    "Silver I",
    "Silver II",
    "Silver III",
    "Silver IV",
    "Silver Elite",
    "Silver Elite Master",
    "Gold Nova I",
    "Gold Nova II",
    "Gold Nova III",
    "Gold Nova Master",
    "Master Guardian I",
    "Master Guardian II",
    "Master Guardian Elite",
    "Distinguished Master Guardian",
    "Legendary Eagle",
    "Legendary Eagle Master",
    "Supreme Master First Class",
    "The Global Elite"
];