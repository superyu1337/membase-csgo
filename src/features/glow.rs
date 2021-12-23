use memflow::VirtualMemory;

use crate::{core::structs::CheatCtx, sdk::{entity::Entity, glowmanager::GlowManager, structs::glow::*}};

pub fn run(ctx: &mut CheatCtx) {
    let glowmanager = GlowManager::get(ctx);
    let local = Entity::get_local(ctx);

    for index in 0..64 {
        let entity = Entity::from_index(ctx, index);
        
        if entity.get_team(ctx) != local.get_team(ctx) {
            let glow_index = entity.get_glowindex(ctx);
            
            // write glow shit
            let offset = (glow_index * 0x38) as usize;

            let color = GlowObjectColor {
                channel_r: 1.0,
                channel_g: 0.25,
                channel_b: 0.25,
                channel_a: 0.75
            };

            let occlusion = GlowObjectOcclusion {
                render_when_occluded: true,
                render_when_unoccluded: true,
            };

            ctx.process.virt_mem.virt_write(glowmanager.ptr + (offset + 0x8), &color).unwrap();
            ctx.process.virt_mem.virt_write(glowmanager.ptr + (offset + 0x28), &occlusion).unwrap();
        }
    }
}