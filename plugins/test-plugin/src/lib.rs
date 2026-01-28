use abi_stable::std_types::{RString, RVec};
use ffi_plugin::{CommandInfo, CommandResult, exec, info, init, set_args, version};
use std::sync::{OnceLock, RwLock};

static COMMAND_INFO: OnceLock<CommandInfo> = OnceLock::new();
static ARGUMENTS: RwLock<Option<RVec<RString>>> = RwLock::new(None);

fn get_plugin_info() -> &'static CommandInfo {
    COMMAND_INFO.get_or_init(|| CommandInfo {
        name: "test-plugins".into(),
        description: "This is a test plugin".into(),
        version: "0.1.0".into(),
    })
}

#[info]
pub fn info() -> CommandInfo {
    get_plugin_info().clone()
}

#[version]
pub fn version() -> RString {
    get_plugin_info().clone().version
}

#[set_args]
pub fn set_args(args: RVec<RString>) {
    let mut guard = ARGUMENTS.write().unwrap();
    *guard = Some(args);

    if let Some(ref args) = *guard {
        println!("Received {} arguments:", args.len());
        for (i, arg) in args.iter().enumerate() {
            println!("  {}: {}", i, arg);
        }
    }
}

#[exec]
pub fn exec() -> CommandResult {
    let args_guard = ARGUMENTS.read().unwrap();

    if let Some(ref args) = *args_guard {
        let message = if args.is_empty() {
            "No arguments provided".into()
        } else {
            let msg = format!("Executed with {} argument(s)", args.len());
            msg.into()
        };

        CommandResult { status: 0, message }
    } else {
        CommandResult {
            status: 1,
            message: "No arguments provided. Call set_args first.".into(),
        }
    }
}

#[init]
pub fn init() {
    get_plugin_info();
    println!("Loaded plugin");
}
