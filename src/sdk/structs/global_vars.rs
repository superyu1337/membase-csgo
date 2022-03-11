#[derive(Clone, Copy)]
#[repr(C)]
pub struct GlobalVars {
    pub realtime: f32,                          // 0x00
	pub framecount: f32,                        // 0x04
	pub absoluteframetime: f32,                 // 0x08
	pub absoluteframestarttimestddev: f32,      // 0x0C
    pub curtime: f32,                           // 0x10
    pub frametime: f32,                         // 0x14
    pub max_clients: i32,                       // 0x18
    pub tickcount: i32,                         // 0x1C
    pub interval_per_tick: f32,                 // 0x20
    pub interpolation_amount: f32,              // 0x24
    pub sim_ticks_this_frame: i32,              // 0x28
    pub network_protocol: i32,                  // 0x2C
    pub save_data: i32,                         // 0x30
    pub client: bool,                           // 0x34
    pub remote_client: bool,                    // 0x35
    pub timestamp_networking_base: i32,         // 0x36
    pub timestamp_randomize_window: i32,        // 0x3A
} // 0x3E

unsafe impl dataview::Pod for GlobalVars {}