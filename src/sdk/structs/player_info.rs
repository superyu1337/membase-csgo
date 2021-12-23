pub struct PlayerInfo {
    unknown: u64,               // 0x0000
    steam_id_64: i64,           // 0x0008 - SteamID64
    name: [char; 128],          // 0x0010 - Player Name
    user_id: i32,               // 0x0090 - Unique Server Identifier
    steam_id_str: [char; 20],   // 0x0094 - STEAM_X:Y:Z
    pad_0x00a8: [u8; 0x10],
    steam_id: u32,              // 0x00B8 - SteamID
    friends_name: [char; 128],
    fakeplayer: u8,
    is_hltv: u8,
}

impl PlayerInfo {
    pub fn get_name(&self) -> String {
        self.name.iter().cloned().collect()
    }
}

unsafe impl dataview::Pod for PlayerInfo {}