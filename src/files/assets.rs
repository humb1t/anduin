use files::{PathType, FileHandle};
use std::option::Option;

// Make sure only one copy of asset is stored in memory
#[allow(dead_code)]
trait AssetsManager {
    fn get<T>(file_name: &str, file_type: Option<T>);
    fn get_by_desc<T>(asset_descriptor: AssetDescriptor<T>);
    fn get_all<T>(file_type: T, out: Vec<T>);
    fn unload(file_name: &str);
    fn contains_asset<T>(asset: T) -> bool;
    fn get_asset_file_name<T>(asset: T) -> String;
    fn is_loaded<T>(file_name: &str, file_type: Option<T>) -> bool;
    fn get_loader<T>(file_name: &str, file_type: Option<PathType>) -> T where T: AssetLoader;
    fn load<T>(file_name: &str, file_type: Option<PathType>, parameter: Option<AssetLoaderParameters<T>>);
    fn load_by_desc<T>(desc: AssetDescriptor<T>);
    fn status() -> Status;
}

struct AssetLoaderParameters<T> {
    param: T
}

struct AssetDescriptor<T>
{
    desc: T
}

pub trait AssetLoader {
    fn resolve(fileName: &'static str) -> FileHandle;
}

enum Status {
    Loading, Caching, Ready, Unloading, Refreshing
}