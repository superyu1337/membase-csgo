use super::vec3::Vec3;

pub struct RadarPlayer {
    pub origin: Vec3,           // 0x0000
    pub viewangles: Vec3,       // 0x000C
    padding_0x0018: [u8; 56],   // 0x0018
    pub health: i32,            // 0x0050
    name: [char; 32],           // 0x0054
    pad_0x00d4: [u8; 117],      // 0x00D4
    pub visible: u8,            // 0x00E9
}   // full size: 0x0B32

impl RadarPlayer {
    pub fn get_name(&self) -> String {
        self.name.iter().cloned().collect()
    }
}

unsafe impl dataview::Pod for RadarPlayer {}