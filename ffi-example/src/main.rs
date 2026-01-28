use abi_stable::std_types::{RString, RVec};
use ffi_interface::CommandRef;
use std::{env, path::PathBuf};

fn main() {
    let plugins_name = ["test_plugin"];
    let binary_path = env::current_exe().expect("Failed to collect binary path");
    let binary_dir = binary_path.parent().unwrap();

    let plugins = plugins_name
        .iter()
        .map(|&name| {
            let plugin_path: PathBuf =
                format!("{}/lib{}.so", binary_dir.to_string_lossy(), name).into();

            let plugin = abi_stable::library::lib_header_from_path(&plugin_path)
                .and_then(|plugin| plugin.init_root_module::<CommandRef>())
                .expect("Failed to load plugin");

            plugin.init()();

            plugin
        })
        .collect::<Vec<CommandRef>>();

    plugins.iter().for_each(|plugin| {
        let info = plugin.info()();
        println!("name       : {}", info.name.to_string());
        println!("description: {}", info.description.to_string());
        println!("version    : {}", info.version.to_string());

        let mut args: RVec<RString> = RVec::new();
        args.push("-h".into());
        args.push("-v".into());

        plugin.set_args()(args);
        let result = plugin.exec()();
        println!("{:?}", result);
    });
}
