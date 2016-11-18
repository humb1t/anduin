mod assets;

use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use std::option::Option;

pub struct Files {}

impl Files {
    pub fn getFileHandle(path: &str, path_type: PathType) -> FileHandle {
        let mut cargo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let buf: PathBuf = match path_type {
            PathType::Internal => {
                cargo.push("resources");
                cargo.push(path);
                cargo
            },
            PathType::Absolute => PathBuf::from(path),
            PathType::Classpath => {
                cargo.push(path);
                cargo
            },
            PathType::Local => {
                cargo.push(path);
                cargo
            },
            PathType::External => PathBuf::from(path),
        };
        let mut f = fs::File::open(&buf).expect("");
        let mut s = String::new();
        f.read_to_string(&mut s);
        println!("{}", s);
        FileHandle {
            file_path: buf,
            fileType: path_type,
        }
    }

    fn internal(path: &str) -> FileHandle {
        Files::getFileHandle(path, PathType::Internal)
    }
    fn classpath(path: &str) -> FileHandle {
        Files::getFileHandle(path, PathType::Classpath)
    }
    fn external(path: &str) -> FileHandle {
        Files::getFileHandle(path, PathType::External)
    }
    fn absolute(path: &str) -> FileHandle {
        Files::getFileHandle(path, PathType::Absolute)
    }
    fn local(path: &str) -> FileHandle {
        Files::getFileHandle(path, PathType::Local)
    }
}

pub trait FileHandleResolver {
    fn resolve(file_name: &str) -> FileHandle;
}

pub struct FileHandle {
    file_path: PathBuf,
    pub fileType: PathType,
}

impl FileHandle {
    pub fn path(&self) -> String {
        let mut file_path: &str;
        match self.file_path.to_str() {
            Some(x) => file_path = x,
            None => file_path = ""
        }
        String::from(file_path).replace("\\", "/")
    }

    /// @return the name of the file, without any parent paths.
    pub fn name(&self) -> String {
        let result = self.file_path.file_name().expect("");
        String::from(result.to_str().expect(""))
    }

    pub fn extension(&self) -> String {
        let result: Option<&str>;
        match self.file_path.extension() {
            Some(x) => result = x.to_str(),
            None => result = None
        }
        String::from(result.expect(""))
    }

    ///** @return the name of the file, without parent paths or the extension.
    pub fn nameWithoutExtension(&self) -> String {
        let result: Option<&str>;
        match self.file_path.file_stem() {
            Some(x) => result = x.to_str(),
            None => result = None
        }
        String::from(result.expect(""))
    }

    ///* @return the path and filename without the extension, e.g. dir/dir2/file.png -> dir/dir2/file. backward slashes will be
    /// *         returned as forward slashes.
    pub fn path_without_extension(&self) -> String {
        let mut file_path: &str;
        match self.file_path.to_str() {
            Some(x) => file_path = x,
            None => file_path = ""
        }
        String::from(file_path).replace(self.name().as_str(), self.nameWithoutExtension().as_str())
    }
}

pub enum PathType {
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