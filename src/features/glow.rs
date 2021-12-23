use memflow::{VirtualMemory};

use crate::{core::structs::CheatCtx, sdk::{entity::Entity, glowmanager::GlowManager, structs::glow::*, engine}};

pub fn run<'a>(ctx: &'a mut CheatCtx) {
    let glowmanager = GlowManager::get(ctx);
    let local = Entity::get_local(ctx);
    let local_team = local.get_team(ctx);
    let globals = engine::get_globalvars(ctx);

    //println!("{:x}", glowmanager.ptr);
    for index in 0..globals.max_clients {
        let entity = Entity::from_index(ctx, index as usize);

        if entity.get_health(ctx) <= 0 {
            continue;
        }
        
        if entity.get_team(ctx) != local_team {

            let glow_index = entity.get_glowindex(ctx);
            
            // write glow shit
            let offset = glow_index * 0x38;

            let color = GlowObjectColor {
                channel_r: 0.25,
                channel_g: 0.25,
                channel_b: 1.0,
                channel_a: 0.75
            };

            let occlusion = GlowObjectOcclusion {
                render_when_occluded: false,
                render_when_unoccluded: true,
            };


            ctx.process.virt_mem.virt_write(glowmanager.ptr + (offset + 0x8), &color).unwrap();
            ctx.process.virt_mem.virt_write(glowmanager.ptr + (offset + 0x28), &occlusion).unwrap();
        }
    }
}