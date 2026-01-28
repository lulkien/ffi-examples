use abi_stable::std_types::RString;
use ffi_plugin::{PluginInfo, info, init, version};
use std::sync::OnceLock;

static PLUGIN_INFO: OnceLock<PluginInfo> = OnceLock::new();

fn get_plugin_info() -> &'static PluginInfo {
    PLUGIN_INFO.get_or_init(|| PluginInfo {
        name: "test-plugins".into(),
        description: "This is a test plugin".into(),
        version: "0.1.0".into(),
    })
}

#[info]
pub fn info() -> PluginInfo {
    get_plugin_info().clone()
}

#[version]
pub fn version() -> RString {
    get_plugin_info().clone().version
}

#[init]
pub fn init() {
    get_plugin_info();
    println!("Loaded plugin");
}
