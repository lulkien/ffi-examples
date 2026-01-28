use abi_stable::std_types::{RString, RVec};
use ffi_plugin::{CommandInfo, CommandResult, exec, info, init, set_args, version};
use std::{
    env::{self, VarError},
    path::PathBuf,
    sync::{OnceLock, RwLock},
};

static COMMAND_INFO: OnceLock<CommandInfo> = OnceLock::new();
static ARGUMENTS: RwLock<Option<RVec<RString>>> = RwLock::new(None);

fn get_plugin_info() -> &'static CommandInfo {
    COMMAND_INFO.get_or_init(|| CommandInfo {
        name: "cd".into(),
        description: "Change working directory".into(),
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
}

#[exec]
pub fn exec() -> CommandResult {
    let args_guard = ARGUMENTS.read().unwrap();
    let mut destination: PathBuf = match env::var("HOME") {
        Ok(value) => value.into(),
        Err(err) => match err {
            VarError::NotPresent => {
                return CommandResult {
                    status: 127,
                    message: "HOME is not set".into(),
                };
            }
            VarError::NotUnicode(_) => {
                return CommandResult {
                    status: 128,
                    message: "Invalid HOME variable".into(),
                };
            }
        },
    };

    if let Some(ref args) = *args_guard {
        if args.len() > 1 {
            return CommandResult {
                status: 1,
                message: "Too many arguments".into(),
            };
        }

        if args.len() == 1 && !args[0].is_empty() {
            // Handle special cases: ~, ~/, and home-relative paths
            let arg = args[0].as_str();
            if arg == "~" {
                // Already set to HOME above
            } else if let Some(relative_path) = arg.strip_prefix("~/") {
                // Home-relative path
                if let Ok(home) = env::var("HOME") {
                    destination = PathBuf::from(home).join(relative_path);
                } else {
                    return CommandResult {
                        status: 127,
                        message: "HOME is not set".into(),
                    };
                }
            } else if arg == "-" {
                // TODO: Handle "cd -" to go to previous directory
                // You'd need to track previous directory in shell state
                return CommandResult {
                    status: 1,
                    message: "cd - not implemented yet".into(),
                };
            } else {
                destination = PathBuf::from(arg);
            }
        }
    }

    let destination = match destination.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            if destination.exists() {
                destination
            } else {
                return CommandResult {
                    status: 2,
                    message: format!("Directory does not exist: {}", destination.display()).into(),
                };
            }
        }
    };

    // Check if destination exists and is a directory
    if !destination.exists() {
        return CommandResult {
            status: 2,
            message: format!("Directory does not exist: {}", destination.display()).into(),
        };
    }

    if !destination.is_dir() {
        return CommandResult {
            status: 3,
            message: format!("Not a directory: {}", destination.display()).into(),
        };
    }

    // Try to change directory
    match env::set_current_dir(&destination) {
        Ok(_) => CommandResult {
            status: 0,
            message: RString::new(),
        },
        Err(e) => CommandResult {
            status: 4,
            message: format!("Cannot cd to {}: {}", destination.display(), e).into(),
        },
    }
}

#[init]
pub fn init() {
    get_plugin_info();
    println!("Loaded cd plugin");
}
