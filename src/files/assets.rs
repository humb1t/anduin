use files::{FileType, FileHandle};

// Make sure only one copy of asset is stored in memory
#[allow(dead_code)]
trait AssetsManager {
    fn get(file_name: &'static str, file_type: FileType);
    fn get_by_desc<T>(assetDescriptor: AssetDescriptor<T>);
    fn getAll<T>(file_type: FileType, out: Vec<T>);
    fn unload(file_name: &'static str);
    fn containsAsset<T>(asset: T) -> bool;
    fn getAssetFileName<T>(asset: T) -> String;
    fn isLoaded(file_name: &'static str, file_type: FileType) -> bool;
    fn getLoader<T>(file_type: FileType, file_name: &'static str) -> T where T: AssetLoader;
    fn load<T>(file_name: &'static str, file_type: FileType, parameter: AssetLoaderParameters<T>);
    fn load_by_desc<T>(desc: AssetDescriptor<T>);
    fn status();
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