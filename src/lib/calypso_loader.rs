use std::collections::HashMap;

use godot::{
    builtin::GString,
    classes::{DirAccess, FileAccess, IObject, Object},
    obj::Base,
    prelude::godot_api,
};
use lib::packages::CalypsoMod;

use crate::*;
const API_VERSION: Version = Version { major: 0, minor: 1, hotfix: 0 };
#[derive(GodotConvert, Var, Export, Debug)]
#[godot(via = GString)]
enum LoaderError {
    #[godot]
    InvalidModPath,
    #[godot]
    InvalidModDirectory,
    #[godot]
    FailedToReadModPath,
    #[godot]
    OK,
    #[godot]
    None,
}
#[derive(GodotClass)]
#[class(base=Object)]
pub struct CalypsoLoader {
    version: Version,
    
    pub mods: HashMap<bool,CalypsoMod>,

    pub base: Base<Object>,
}
#[godot_api]
impl IObject for CalypsoLoader {
    fn init(base: Base<Object>) -> Self {
        CalypsoLoader {version: API_VERSION ,base,mods:HashMap::new() }
    }
}

#[godot_api]
impl CalypsoLoader {
    #[func]
    fn setup(&mut self) {}
    #[func]
    fn set_mod_path(&mut self, path: GString) -> LoaderError {
        let my_file = DirAccess::open(path.clone());
        if my_file.is_some() {
            godot_print!("Calypso: it work");
            for dir_name in my_file.unwrap().get_directories().to_vec() {
                godot_print!("current dir is {dir_name}");

                let full_path = format!("{:?}/{:?}", path.clone(), dir_name.clone());
                godot_print!("attempting to open: {path}{dir_name}");

                let sub_path = DirAccess::open(full_path.clone().into());

                match sub_path {
                    Some(mut sub_dir) => {
                        for file in sub_dir.get_files().to_vec() {
                            if file.to_string().ends_with(".cy")
                                & !file.to_string().starts_with(".")
                            {
                                let mod_path = format!("{:?}/{:?}", full_path.clone(), file);
                                godot_print!("found mod file! {}", mod_path.clone());
                                let new_mod: ModPackage = match read_mod_config(mod_path.clone().into()) {
                                    Ok(package) => {
                                        godot_print!("loaded mod {file}");
                                        package
                                    }
                                    Err(_) => {
                                        godot_error!("Failed to load the mod {:?}",mod_path.clone());
                                        return LoaderError::FailedToReadModPath;
                                    }
                                };
                                let module = CalypsoMod { package: new_mod };
                                self.mods.insert(false, module);
                            }
                        }
                    }
                    None => godot_error!("Could not find mod folder"),
                }
            }
            return LoaderError::InvalidModPath;
        } else {
            godot_error!("Invalid mod directory, is it a valid path?");
            return LoaderError::InvalidModDirectory;
        }
    }
    /// prints a list of every mod configuration (*.cy) currently avalible
    #[func]
    fn get_enabled_mods(&mut self) {
        for module in &self.mods {
            godot_print!("{module:#?}")
        }
    }
}

fn read_mod_config(gd_path: GString) -> Result<ModPackage, LoaderError> {
    let content = FileAccess::get_file_as_string(gd_path);
    let config: ModPackage = toml::from_str(&content.to_string())
        .to_owned()
        .map_err(|_e| LoaderError::InvalidModPath)?;
    return Ok(config);
}

