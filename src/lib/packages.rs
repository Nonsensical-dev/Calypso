use godot::{
    builtin::GString, classes::{IResource, Resource}, meta::ToGodot, obj::Base, prelude::{godot_api, GodotClass, GodotConvert, Var}
};
use serde::{Deserialize, Serialize};

use crate::Version;


#[derive(GodotClass, Debug)]
#[class(base=Resource)]
pub struct CalypsoMod {
    pub package: ModPackage,
    //pub base: Base<Resource>,
}
#[godot_api]
impl IResource for CalypsoMod {
    fn init(_base: Base<Resource>) -> Self {
        CalypsoMod {
            package: ModPackage::default(),
        }
    }
}
#[godot_api]
impl CalypsoMod {
    ///reads mod
    #[func]
    fn read(&mut self, path: GString) {
        godot::prelude::godot_print!("reading path {path:#?}")
    }
    #[func]
    fn get_name(&self) -> GString {
        self.package.name.clone().into()

    }
    #[func]
    fn get_description(&self) -> GString  {
        match &self.package.mod_info {
            Some(info) => if info.description.is_some() {
                info.description.clone().unwrap().into()
            } else {
                "None".into()
            }
            None => "None".into(),
        }
        
    }
    #[func]
    fn get_api_ver(&self) -> GString  {
        format!("{}{}{}",self.package.api_version.major,self.package.api_version.minor,self.package.api_version.hotfix).into()
        
    }
    #[func]
    fn get_mod_ver(&self) -> GString  {
        format!("{}{}{}",self.package.mod_version.major,self.package.mod_version.minor,self.package.mod_version.hotfix).into()
    }
    #[func]
    fn get_author(&self) -> GString  {
        match &self.package.mod_info {
            Some(info) => if info.author.is_some() {
                info.author.clone().unwrap().into()
            } else {
                "None".into()
            }
            None => "None".into(),
        }
    }
    #[func]
    fn get_license(&self) -> GString  {
        self.package.license.to_owned().to_godot()
    }

}
#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all="snake_case")]
pub struct ModPackage {
    pub name: String,
    pub dependencies: Option<Vec<String>>,
    pub mod_version: Version,
    pub api_version: Version,
    pub license: License,
    pub mod_info: Option<PackageInfo>,
}
#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all="snake_case")]
pub struct PackageInfo {
    pub author: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all="PascalCase")]
#[derive(GodotConvert, Var)]
#[godot(via = GString)]
pub enum License {
    GNUAGPLv3,
    GNUGPLv3,
    GNULGPLv3,
    MPLv3,
    Apache2,
    #[default]
    MIT,
    Boost1,
    PublicDomain,
    ClosedSource,
    Other,
}
