use abi_stable::{
    StableAbi, declare_root_module_statics, library::RootModule, package_version_strings,
    sabi_types::VersionStrings, std_types::RString,
};

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = PluginRef)))]
#[sabi(missing_field(panic))]
pub struct Plugin {
    pub init: extern "C" fn(),
    pub info: extern "C" fn() -> PluginInfo,
    pub version: extern "C" fn() -> RString,
}

#[repr(C)]
#[derive(StableAbi, Debug, Clone)]
pub struct PluginInfo {
    pub name: RString,
    pub description: RString,
    pub version: RString,
}

impl RootModule for PluginRef {
    declare_root_module_statics! {PluginRef}

    const BASE_NAME: &'static str = "ffi_plugin";
    const NAME: &'static str = "ffi_plugin";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}
