pub mod plugin;

use abi_stable::{
    external_types::RRwLock,
    std_types::{RArc, RVec},
};
use plugin::Plugin;

pub type GState = RArc<RRwLock<GlobalState>>;

#[repr(C)]
pub struct GlobalState {
    pub plugins: RVec<RArc<RRwLock<Plugin>>>,
}