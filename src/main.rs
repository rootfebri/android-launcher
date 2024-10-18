use std::path::PathBuf;
use crate::adb::Emulator;

mod adb;

static EM_HOME: once_cell::sync::Lazy<PathBuf> = once_cell::sync::Lazy::new(|| {
    let path = std::env::var("ANDROID_HOME");
    if let Ok(p) = path {
        let pb = PathBuf::from(p);
        if pb.exists() && pb.is_dir() {
            pb
        } else {
            panic!("ANDROID_HOME is not a directory or does not exists")
        }
        
    } else { 
        panic!("ANDROID_HOME must be set, check your installation or ANDROID_HOME in your PATH")
    }
});

fn main() {
    dotenvy::dotenv().ok();
    Emulator::new().select().unwrap().launch().ok();
}
