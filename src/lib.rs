#![crate_type = "cdylib"]
use godot::{classes::Engine, prelude::*};
use lib::{
    calypso_loader,
    packages::{ModPackage, PackageInfo},
};
use serde::{Deserialize, Serialize};

struct LibCalypso {}
pub mod lib {
    pub mod calypso_loader;
    pub mod packages;
}


#[gdextension]
unsafe impl ExtensionLibrary for LibCalypso {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            Engine::singleton().register_singleton(
                StringName::from("CalypsoLoader"),
                calypso_loader::CalypsoLoader::new_alloc().upcast::<Object>(),
            );
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            let singleton_name = StringName::from("CalypsoLoader");
            let singleton = engine
                .get_singleton(singleton_name.clone())
                .expect("cannot retrieve the singleton");   
            engine.unregister_singleton(singleton_name);
            singleton.free();
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all="snake_case")]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub hotfix: u32,
}
impl ModPackage {
    pub fn new(
        name: &str,
        deps: Option<Vec<String>>,
        info: Option<PackageInfo>,
        version: Version,
        api_version: Version,
    ) -> Self {
        Self {
            name: String::from(name).to_lowercase(),
            dependencies: deps,
            mod_version: version,
            api_version: api_version,
            mod_info: info,
            license: lib::packages::License::MIT,
        }
    }
}
