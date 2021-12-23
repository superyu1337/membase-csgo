#[derive(Clone, Copy)]
pub struct GlowObjectColor {
    pub channel_r: f32,
    pub channel_g: f32,
    pub channel_b: f32,
    pub channel_a: f32,
}

unsafe impl dataview::Pod for GlowObjectColor {}

pub struct GlowObjectOcclusion {
    pub render_when_occluded: bool,
    pub render_when_unoccluded: bool,
}

unsafe impl dataview::Pod for GlowObjectOcclusion {}