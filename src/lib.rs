#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![doc = include_str!("../README.md")]

use nskeyedarchiver_converter::Converter;
use std::{
    cell::RefCell,
    fs,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};
use zip::read::ZipArchive;

pub struct File {
    path: PathBuf,
    archive: RefCell<ZipArchive<BufReader<fs::File>>>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occurred while opening the file.
    #[error("Could not open file: {0}")]
    FileOpen(#[from] std::io::Error),

    /// An error occurred while parsing the file.
    #[error("Could not parse file: {0}")]
    Zip(#[from] zip::result::ZipError),

    /// An error occurred while unpacking the file's metadata.
    #[error("Could not unpack the file's metadata: {0}")]
    NSKeyedArchiver(#[from] nskeyedarchiver_converter::ConverterError),

    /// An error occurred while parsing the file's metadata.
    #[error("Could not parse the file's metadata: {0}")]
    Plist(#[from] plist::Error),
}

impl File {
    /// Open a Procreate file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file could not be opened.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref().to_path_buf();

        let file = fs::File::open(&path)
            .map(BufReader::new)
            .map_err(Error::FileOpen)?;

        Ok(Self {
            path,
            archive: RefCell::new(ZipArchive::new(file)?),
        })
    }

    /// Get the file's metadata as a plist.
    ///
    /// # Errors
    ///
    /// Returns an error if the file's metadata could not be unpacked or parsed.
    pub fn metadata(&self) -> Result<plist::Value, Error> {
        let mut archive = self.archive.borrow_mut();

        let plist = archive
            .by_name("Document.archive")?
            .bytes()
            .collect::<Result<Vec<_>, _>>()?;

        let plist: plist::Value = Converter::from_bytes(&plist)?.decode()?;

        Ok(plist)
    }

    /// Get a thumbnail of the file.
    ///
    /// # Errors
    ///
    /// Returns an error if the thumbnail could not be read.
    pub fn thumbnail(&self) -> Result<Vec<u8>, Error> {
        let mut archive = self.archive.borrow_mut();

        let thumbnail = archive
            .by_name("QuickLook/Thumbnail.png")?
            .bytes()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(thumbnail)
    }

    /// Get a list of mp4 segments that make up the file's timelapse, as bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the segments could not be read.
    pub fn timelapse_segments(&self) -> Result<Vec<Vec<u8>>, Error> {
        let mut archive = self.archive.borrow_mut();

        let mut segments = archive
            .file_names()
            .filter(|name| name.starts_with("video/segments/"))
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        segments.sort_unstable_by(|a, b| {
            let a: u32 = a
                .split('-')
                .last()
                .and_then(|s| s.split('.').next())
                .map_or_else(|| unreachable!(), str::parse)
                .unwrap_or_else(|_| unreachable!());

            let b: u32 = b
                .split('-')
                .last()
                .and_then(|s| s.split('.').next())
                .map_or_else(|| unreachable!(), str::parse)
                .unwrap_or_else(|_| unreachable!());

            a.cmp(&b)
        });

        let segments = segments
            .iter()
            .map(|name| {
                archive
                    .by_name(name)?
                    .bytes()
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(segments)
    }
}

impl Clone for File {
    fn clone(&self) -> Self {
        Self::open(&self.path).expect("Could not clone file")
    }
}
