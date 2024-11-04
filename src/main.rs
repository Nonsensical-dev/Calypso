#[warn(dead_code)]
use calypso::*;
use lib::packages::{ModPackage, PackageInfo};
use std::{fs::File, io::Write};
fn main() {
    let api_version = Version {
            major: 0,
            minor: 1,
            hotfix: 0,
        };
    let new_mod = ModPackage::new(
        "Calypso-loader",
        None,
        Some(PackageInfo {
            author: Some("Caznix".to_string()),
            description: None,
        }),
        Version {
            major: 0,
            minor: 1,
            hotfix: 1,
        },
        api_version,
    );
    let string = toml::to_string(&new_mod).unwrap();
    let mut file = File::create("./new_mod.cy").unwrap();
    file.write_all(string.as_bytes()).unwrap();

    println!("{string:#}");
}
