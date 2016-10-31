mod assets;

use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

pub struct Files {}

impl Files {
    fn getFileHandle(path: &'static str, fileType: FileType) -> FileHandle {unimplemented!()}

    fn internal(path: &'static str) -> FileHandle {unimplemented!()}

    fn classpath(path: &'static str) -> FileHandle {unimplemented!()}
    fn external(path: &'static str) -> FileHandle {unimplemented!()}
    fn absolute(path: &'static str) -> FileHandle {unimplemented!()}
    fn local(path: &'static str) -> FileHandle {
        unimplemented!()
    }
}

pub trait FileHandleResolver {
    fn resolve(file_name: &'static str) -> FileHandle;
}

pub struct FileHandle {
    file_path: PathBuf,
    file: fs::File,
    pub fileType: FileType,
}
/*
impl FileHandle {
    fn path(&self) -> String {
        let mut file_path: &str;
        match self.file_path.as_ref().to_str() {
            Some(x) => file_path = x,
            None => file_path = ""
        }
        String::from(file_path).replace("\\", "/")
    }

    /// @return the name of the file, without any parent paths.
    fn name(&self) -> String {
        self.file_path.as_ref().file_name().as_mut();
    }

    fn extension(&self) -> String {
        match self.file_path.as_ref().extension() {
            Some(x) => x,
            None => ""
        }
    }

    /** @return the name of the file, without parent paths or the extension. */
    fn nameWithoutExtension(&self) -> String {
        match self.file_path.as_ref().file_stem() {
            Some(x) => x,
            None => ""
        }
    }

    /** @return the path and filename without the extension, e.g. dir/dir2/file.png -> dir/dir2/file. backward slashes will be
     *         returned as forward slashes. */
    fn path_without_extension(&self) -> String {
        let mut file_path: &str;
        match self.file_path.as_ref().to_str() {
            Some(x) => file_path = x,
            None => file_path = ""
        }
        String::from(file_path).replace(self.name(), self.nameWithoutExtension().as_str())
    }
}*/
/*
trait Files {
    fn getFileHandle(path: &'static str, fileType: FileType) -> FileHandle;

    /** Convenience method that returns a {@link FileType#Classpath} file handle. */
    fn classpath(path: &'static str) -> FileHandle;

    /** Convenience method that returns a {@link FileType#Internal} file handle. */
    fn internal(path: &'static str) -> FileHandle;

    /** Convenience method that returns a {@link FileType#External} file handle. */
    fn external(path: &'static str) -> FileHandle;

    /** Convenience method that returns a {@link FileType#Absolute} file handle. */
    fn absolute(path: &'static str) -> FileHandle;

    /** Convenience method that returns a {@link FileType#Local} file handle. */
    fn local(path: &'static str) -> FileHandle;

    /** Returns the external storage path directory. This is the SD card on Android and the home directory of the current user on
     * the desktop. */
    fn getExternalStoragePath() -> String;

    /** Returns true if the external storage is ready for file IO. Eg, on Android, the SD card is not available when mounted for use
     * with a PC. */
    fn isExternalStorageAvailable() -> bool;

    /** Returns the local storage path directory. This is the private files directory on Android and the directory of the jar on the
     * desktop. */
    fn getLocalStoragePath() -> String;

    /** Returns true if the local storage is ready for file IO. */
    fn isLocalStorageAvailable() -> bool;
}
*/

pub enum FileType {
    /** Path relative to the root of the classpath. Classpath files are always readonly. Note that classpath files are not
     * compatible with some functionality on Android, such as {@link Audio#newSound(FileHandle)} and
     * {@link Audio#newMusic(FileHandle)}. */
    Classpath,

    /** Path relative to the asset directory on Android and to the application's root directory on the desktop. On the desktop,
     * if the file is not found, then the classpath is checked. This enables files to be found when using JWS or applets.
     * Internal files are always readonly. */
    Internal,

    /** Path relative to the root of the SD card on Android and to the home directory of the current user on the desktop. */
    External,

    /** Path that is a fully qualified, absolute filesystem path. To ensure portability across platforms use absolute files only
     * when absolutely (heh) necessary. */
    Absolute,

    /** Path relative to the private files directory on Android and to the application's root directory on the desktop. */
    Local,
}