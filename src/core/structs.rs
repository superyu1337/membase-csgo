use memflow::{VirtualDMA, ConnectorInstance, DirectTranslate, CachedMemoryAccess, TimedCacheValidator, CachedVirtualTranslate};
use memflow_win32::{Win32Process, Win32ModuleInfo, Win32VirtualTranslate};

use crate::sdk::structs::vec3::Vec3;
use crate::core::offset_manager::OffsetManagerOutput;

pub struct CheatCtx<'a> {
    pub process: Win32Process<VirtualDMA<CachedMemoryAccess<'a, ConnectorInstance, TimedCacheValidator>, CachedVirtualTranslate<DirectTranslate, TimedCacheValidator>, Win32VirtualTranslate>>,
    pub engine_module: Win32ModuleInfo,
    pub client_module: Win32ModuleInfo,
    pub offsets: OffsetManagerOutput,
    pub config: Config,
}

#[derive(Clone, Copy)]
pub struct Config {
    pub glow: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { glow: false }
    }
}

impl Config {
    pub fn valuestr_at_index(&self, index: usize) -> Option<String> {
        match index {
            0 => {
                return Some(format!("{}", self.glow));
            }
            _ => { None }
        }
    }

    pub fn change_at_index(&mut self, index: usize, _change_type: i8) {
        match index {
            0 => {
                self.glow = !self.glow;
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerData {
    pub name: [u8; 128],
    pub rank: i32,
    pub rank_name: [u8; 32],
    pub pos: Vec3,
    pub team: i32
}


#[derive(Clone, Copy)]
pub struct ThreadMsg {
    pub exited: bool,
    pub new_config: Option<Config>,
    pub playerdata_array: [Option<PlayerData>; 64],
    pub average_execution_time: Option<u128>
}