extern crate cc;
extern crate proc_macro2;
extern crate syn;

use std::path::{Path, PathBuf};
use syn::Item;

mod generate;
mod extract;
use extract::extract_asm;

use std::fs::{read_dir, File};
use std::io::Read;
use std::env;

#[macro_export]
macro_rules! lasm {
    ($abi:tt fn $name:tt -> %$ret:tt {
        $($dontcare:tt)*
    } $($more:tt)*) => (lasm!($($more)*););
    ($abi:tt fn $name:tt {
        $($dontcare:tt)*
    } $($more:tt)*) => (lasm!($($more)*););

    ($abi:tt fn $name:tt -> $ret:tt {
        $($dontcare:tt)*
    } $($more:tt)*) => (lasm!($($more)*););
    ($abi:tt fn $name:tt {
        $($dontcare:tt)*
    } $($more:tt)*) => (lasm!($($more)*););

    () => {};
}

#[derive(Clone, Debug, PartialEq)]
pub struct Asm {
    pub name: String,
    pub ret: Option<String>,
    pub body: Vec<String>,
}

pub fn parse_file(code: String) {
    let syntax = syn::parse_file(&code).expect("Unable to parse file");
    for item in syntax.items {
        match item {
            Item::Macro(macro_item) => {
                let mac = macro_item.mac;
                if mac.path == "lasm".into() {
                    extract_asm(mac.tts).map(|x| x.generate());
                }
            }
            _ => {}
        }
    }
}

pub fn parse_dir(dir: &Path) {
    for item in read_dir(dir).unwrap() {
        match item {
            Ok(item) => {
                let item_type = item.file_type().unwrap();
                if item_type.is_dir() {
                    parse_dir(&item.path())
                } else if item.path().to_str().unwrap().ends_with(".rs") {
                    let mut file = File::open(item.path()).unwrap();
                    let mut content = String::new();
                    file.read_to_string(&mut content).unwrap();
                    parse_file(content);
                }
            }
            _ => println!("cargo:warning=unable to read all source files"),
        }
    }
}

pub fn parse() {
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src");
    parse_dir(&dir);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
