use core::{fmt, slice};
use std::{ffi::CString, fmt::Debug, fs, io::{BufReader, Cursor, Read, Write}, os::unix::fs::MetadataExt, path::PathBuf};
use hex_literal::hex;
use chrono::DateTime;
use sha1::{Sha1, Digest};

use crate::{args::Warp, blob::Blob};

pub const INDEX_DATA: &[u8] = &hex!(
    "44 49 52 43 00 00 00 02 00 00 00 05 67 f1 65 1b
    32 01 66 9e 67 f1 65 1b 32 01 66 9e 00 00 08 02
    00 92 bc db 00 00 81 a4 00 00 03 e8 00 00 03 e8
    00 00 00 0d 77 ef 3b bc 6c 33 3c 60 88 eb a7 a7
    b0 c4 c2 62 03 ed 97 65 00 09 66 69 6c 65 61 2e
    74 78 74 00 67 f1 65 a3 2a 45 0f 65 67 f1 65 a3
    2a 45 0f 65 00 00 08 02 00 92 bc dd 00 00 81 a4
    00 00 03 e8 00 00 03 e8 00 00 00 0b d0 26 4d 77
    65 9d c7 e6 f7 ad 0a 62 19 8a 15 7e 95 7b b9 2a
    00 09 66 69 6c 65 62 2e 74 78 74 00 67 f1 74 22
    05 e5 db a4 67 f1 74 22 05 e5 db a4 00 00 08 02
    00 a0 ba dc 00 00 81 a4 00 00 03 e8 00 00 03 e8
    00 00 00 05 92 ba ab df 84 82 1c 93 c8 8a 45 a4
    34 fb 4e 88 ce 0d 08 c4 00 13 73 72 63 2f 64 62
    2f 70 6f 73 74 67 72 65 73 2e 74 78 74 00 00 00
    00 00 00 00 67 f1 70 7e 1e 98 00 d6 67 f1 70 7e
    1e 98 00 d6 00 00 08 02 00 9c b8 06 00 00 81 a4
    00 00 03 e8 00 00 03 e8 00 00 00 0b 85 cb 6c 13
    14 79 5d 06 61 0b f8 bd cb 88 cb f7 c5 99 49 28
    00 0d 73 72 63 2f 66 69 6c 65 63 2e 74 78 74 00
    00 00 00 00 67 f1 7a cf 26 b7 26 95 67 f1 7a cf
    26 b7 26 95 00 00 08 02 00 9c b8 08 00 00 81 a4
    00 00 03 e8 00 00 03 e8 00 00 00 05 b2 5f a3 fc
    47 3b 6e fd 5d ed 03 bc dd bc 4d 37 fc 20 67 4b
    00 0d 7a 65 64 2f 66 69 6c 65 64 2e 74 78 74 00
    00 00 00 00 54 52 45 45 00 00 00 6c 00 35 20 32
    0a d6 5f 1e 52 7c 08 b2 21 de 80 43 51 61 5c 52
    7d 18 1b 10 2f 73 72 63 00 32 20 31 0a 91 dc 0d
    3e 30 ad 86 93 c1 0e d2 3b a0 21 ed 2b e3 5c f0
    28 64 62 00 31 20 30 0a 13 bb 37 bc 8c a7 10 20
    99 91 8f 1a 7e 96 d4 a7 87 3a f4 38 7a 65 64 00
    31 20 30 0a d0 39 fd 2d 7f 69 fe 87 ac 3e 6b 53
    94 0a ec ff 72 ab 7a e6 53 b5 bf c2 83 67 bc 8a
    af 62 e7 6f 76 ff a5 56 7f cd 2f ec");

pub const NO_TREE: &[u8] = &hex!("
    444952430000000200000003680604fb049d5c37680604fb049d5c370000
    0802008488dc000081a4000003e8000003e80000000b303ff981c488b812
    b6215f7db7920dedb3b59d9a000966696c65612e74787400680605312b56
    bb72680605312b56bb7200000802008488de000081a4000003e8000003e8
    0000000c1c59427adc4b205a270d8f810310394962e79a8b000966696c65
    622e747874006806057c2ba31aaa6806057c2ba31aaa000008020086c9c1
    000081a4000003e8000003e80000000b667bb3858a056cc96e79c0c3b1ed
    fb60135c2359000d7372632f66696c65632e74787400000000001238db55
    255645bdf17b967d57cc8ecf6015ffae
    ");

#[derive(Clone, PartialEq, Eq)]
pub struct IndexHeader {
    pub signature: [u8; 4],
    pub version: u32,
    pub entry_count: u32
}

impl IndexHeader {
    fn new(signature: [u8; 4], version: u32, entry_count: u32) -> Self {
        Self {
            signature,
            version,
            entry_count
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(self.signature);
        bytes.extend(self.version.to_be_bytes());
        bytes.extend(self.entry_count.to_be_bytes());

        bytes

    }
}

impl fmt::Debug for IndexHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndexHeader")
         .field("signature", &String::from_utf8_lossy(&self.signature))
         .field("version", &self.version)
         .field("entry_count", &self.entry_count)
         .finish()
    }
}

impl TryFrom<&mut Cursor<&[u8]>> for IndexHeader {
    type Error = IndexParseError;

    fn try_from(reader: &mut Cursor<&[u8]>) -> Result<Self, Self::Error> {
        let mut signature = [0u8; 4];
        let mut version = [0u8; 4];
        let mut index_count = [0u8; 4];

        reader.read_exact(&mut signature);
        reader.read_exact(&mut version);
        reader.read_exact(&mut index_count);

        let version: u32 = u32::from_be_bytes(version);
        let index_count: u32 = u32::from_be_bytes(index_count);

        Ok(IndexHeader::new(signature, version, index_count))
    }
}


#[derive(Clone)]
pub struct IndexEntry {
    pub ctime_seconds: u32,
    pub ctime_nanoseconds: u32,
    pub mtime_seconds: u32,
    pub mtime_nanoseconds: u32,
    pub dev: u32,
    pub ino: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub filesize: u32,
    pub sha: [u8; 20],
    pub flags: u16,
    pub path: String
    // 1 - 8 bytes nul bytes necessary to pad the entry.
}

impl IndexEntry {
    pub fn entry_from_file(file: PathBuf) -> IndexEntry {
        let metadata = fs::metadata(&file).expect("Unable to get metadata about this file");
        let blob: Blob = Blob::new(file.clone());
        let blob_sha: String = blob.hash_object().unwrap();
        let sha_bytes = hex::decode(&blob_sha).expect("Invalid hex in SHA");
        let mut sha = [0u8; 20];
        sha.copy_from_slice(&sha_bytes);
        blob.compress_to_object();

        Self {
            ctime_seconds: metadata.ctime() as u32,
            ctime_nanoseconds: metadata.ctime_nsec() as u32,
            mtime_seconds: metadata.mtime() as u32,
            mtime_nanoseconds: metadata.mtime_nsec() as u32,
            dev: metadata.dev() as u32,
            ino: metadata.ino() as u32,
            mode: metadata.mode(),
            uid: metadata.uid(),
            gid: metadata.gid(),
            filesize: metadata.len() as u32,
            sha,
            flags: file.file_name().unwrap().len() as u16,
            path: file.file_name().unwrap().to_string_lossy().to_string(),
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(&self.ctime_seconds.to_be_bytes());
        bytes.extend(&self.ctime_nanoseconds.to_be_bytes());
        bytes.extend(&self.mtime_seconds.to_be_bytes());
        bytes.extend(&self.mtime_nanoseconds.to_be_bytes());
        
        bytes.extend(&self.dev.to_be_bytes());
        bytes.extend(&self.ino.to_be_bytes());

        bytes.extend(&self.mode.to_be_bytes());

        bytes.extend(&self.uid.to_be_bytes());
        bytes.extend(&self.gid.to_be_bytes());

        bytes.extend(&self.filesize.to_be_bytes());

        bytes.extend(&self.sha);
        bytes.extend(&self.flags.to_be_bytes());

        bytes.extend(self.path.as_bytes());
        bytes.push(0); // Null terminator
        
        // Pad to multiple of 8 bytes
        let padding = (8 - (bytes.len() % 8)) % 8;
        bytes.extend(vec![0; padding]);
        
        bytes
    }
}


impl Debug for IndexEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndexEntry")
         .field("ctime_seconds", &DateTime::from_timestamp(self.ctime_seconds.into(), 0).unwrap())
         .field("ctime_nanoseconds", &DateTime::from_timestamp(self.ctime_seconds.into(), self.ctime_nanoseconds.into()).unwrap())
         .field("mtime_seconds", &DateTime::from_timestamp(self.mtime_seconds.into(), 0).unwrap())
         .field("mtime_nanoseconds", &DateTime::from_timestamp(self.mtime_seconds.into(), self.mtime_nanoseconds.into()).unwrap())
         .field("dev", &self.dev)
         .field("ino", &self.ino)
         .field("mode", &format!("{:o}", &self.mode))
         .field("uid", &self.uid)
         .field("gid", &self.gid)
         .field("filesize", &self.filesize)
         .field("sha", &self.sha.iter().map(|b| format!("{:02x}", b)).collect::<String>())
         .field("flags", &self.flags)
         .field("path", &self.path)
         .finish()
    }
}

impl TryFrom<&mut Cursor<&[u8]>> for IndexEntry {
    type Error = String; // I will write the error logic some other time lol(Everything seems to work fine for now).

    fn try_from(reader: &mut Cursor<&[u8]>) -> Result<Self, Self::Error> {
        let mut buffer = [0u8; 40];
        reader.read_exact(&mut buffer);

        let values = buffer.chunks_exact(4).map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap())).collect::<Vec<u32>>();
        let mut sha = [0u8; 20];
        reader.read_exact(&mut sha);

        let mut flags: [u8; 2] = [0u8; 2];
        reader.read_exact(&mut flags);

        let pathname_length: u16 = u16::from_be_bytes(flags);

        let mut path = vec![0u8; pathname_length as usize];
        reader.read_exact(&mut path);

        let mut single_byte = [0u8; 1];
        while let Ok(_) = reader.read_exact(&mut single_byte) {
            if single_byte[0] != 0 {
                let current_pos = reader.position();
                reader.set_position(current_pos - 1);
                break;
            }
        }


        let entry_one = IndexEntry { ctime_seconds: values[0], ctime_nanoseconds: values[1], mtime_seconds: values[2], mtime_nanoseconds: values[3], dev: values[4], ino: values[5], mode: values[6], uid: values[7], gid: values[8], filesize: values[9], sha, flags: u16::from_be_bytes(flags), path: String::from_utf8(path).unwrap() };
        Ok(entry_one)
    }
}

#[derive(Clone)]
pub struct IndexExtension {
    signature: [u8; 4],
    extension_size: u32,
    extension_data: CacheTreeEntry
}

impl IndexExtension {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(&self.signature);
        bytes.extend(&self.extension_size.to_be_bytes());
        self.extension_data.to_bytes(&mut bytes);

        bytes
    }
}

impl Debug for IndexExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndexExtension")
         .field("signature", &String::from_utf8_lossy(&self.signature))
         .field("extension_size", &self.extension_size)
         .field("extension_data", &self.extension_data)
         .finish()
    }
}
impl TryFrom<&mut Cursor<&[u8]>> for IndexExtension {
    type Error = String;

    fn try_from(reader: &mut Cursor<&[u8]>) -> Result<Self, Self::Error> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer);

        let signature = buffer;

        reader.read_exact(&mut buffer);
        let extension_size = u32::from_be_bytes(buffer);


        let mut extension_data = vec![0u8; extension_size as usize];
        reader.read_exact(&mut extension_data);

        let cache_entry = CacheTreeEntry::try_from(extension_data).unwrap();

        Ok(IndexExtension { signature, extension_size, extension_data: cache_entry})
    }
}

#[derive(Clone)]
pub struct CacheTreeEntry {
    pub path: Vec<u8>,
    pub entry_count: u8,
    pub subtree_count: u8,
    pub sha: [u8; 20],
    pub subtrees: Option<Vec<CacheTreeEntry>>
}

impl CacheTreeEntry {
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Vec<u8> {
        // let mut bytes = Vec::new();

        bytes.extend(&self.path);
        bytes.extend(&(self.entry_count + 48).to_be_bytes());
        bytes.extend((32 as u8).to_be_bytes());
        bytes.extend(&(self.subtree_count + 48).to_be_bytes());
        bytes.extend((10 as u8).to_be_bytes());
        bytes.extend(&self.sha);

        if let None = self.subtrees {
            return bytes.to_vec();
        } else if let Some(subtrees) = &self.subtrees {
            for subtree in subtrees {
                subtree.to_bytes(bytes);
            }
        } 
        bytes.to_vec()
    }
}

impl Debug for CacheTreeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CacheTreeEntry")
         .field("path", &self.path)
         .field("entry_count", &self.entry_count)
         .field("subtree_count", &self.subtree_count)
         .field("sha", &self.sha.iter().map(|ch| format!("{:02x}", ch)).collect::<String>())
         .field("subtrees", &self.subtrees)
         .finish()
    }
}

fn create_cache(reader: &mut BufReader<&[u8]>) -> CacheTreeEntry {
    let mut single_byte = [0u8; 1];
    let mut path = String::new();
    // Nul terminated path component
    while let Ok(_) = reader.read_exact(&mut single_byte) {
        if single_byte[0] == 0 {
            break;
        } else {
            path.push(single_byte[0].into());
        }
    }

    let new_path = CString::new(path).unwrap();
    let x = new_path.as_bytes_with_nul().to_owned();

    // ASCII Entry count
    reader.read_exact(&mut single_byte);
    let entry_count = u8::from_be_bytes(single_byte) - 48;

    // ASCII Space
    reader.read_exact(&mut single_byte);

    // ASCII number of subtrees
    reader.read_exact(&mut single_byte);
    let subtree_count = u8::from_be_bytes(single_byte) - 48;
    
    let subtrees: Option<Vec<CacheTreeEntry>>;

    // ASCII newline
    reader.read_exact(&mut single_byte);
    let mut sha = [0u8; 20];
    
    // SHA tree object
    reader.read_exact(&mut sha);

    if subtree_count > 0 {
        let mut trees: Vec<CacheTreeEntry> = Vec::new();
        for _ in 0..subtree_count {
            trees.push(create_cache(reader));
        }

        subtrees = Some(trees);
    } else {
        subtrees = None;
    }
    

    return CacheTreeEntry { path: x, entry_count, subtree_count, sha, subtrees };
}


impl TryFrom<Vec<u8>> for CacheTreeEntry {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value.as_slice());
        Ok(create_cache(&mut reader))
    }
}

#[derive(Debug)]
pub enum IndexParseError {
    InvalidSignature,
    UnsuppoertedVersion(u32),
    ChecksumMismatch,
    InvalidEntry,
    Io,
    InvalidExtension
}

pub struct WarpIndex {
    pub header: IndexHeader,
    pub entries: Vec<IndexEntry>,
    pub extensions: Option<IndexExtension>,
    pub checksum: [u8; 20]
}

fn generic_index() -> PathBuf{
    let mut root = std::env::current_dir().expect("Unable to get the current working directory");
    root.push(".warp");
    root.push("index");
    root
}

pub fn index_file_exists() -> bool {
    let root = generic_index();

    root.is_file()
}

pub fn create_new_index() -> fs::File {
    let root = generic_index();
    fs::File::create(root).expect("Unable to create a index file")
}
impl WarpIndex {
    pub fn without_extension(entries: Vec<IndexEntry>) -> Self {
        let new_index_header = IndexHeader::new([68, 73, 82, 67], 2, entries.len() as u32);
        let mut hasher = Sha1::new();

        let mut bytes = Vec::new();
        bytes.extend(new_index_header.to_bytes());
        entries.iter().for_each(|entry| bytes.extend(entry.to_bytes()));
        hasher.update(bytes);
        let mut checksum = [0u8; 20];

        let bytes = hasher.finalize();
        checksum.copy_from_slice(bytes.as_slice());

        WarpIndex { header: new_index_header, entries, extensions: None, checksum }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let index_header_bytes = self.header.to_bytes();
        let index_entry_bytes = self.entries.iter().map(|entry| entry.to_bytes()).collect::<Vec<_>>().concat();
        let extension_bytes = match &self.extensions {
            Some(ext) => ext.to_bytes(),
            None => Vec::new()
        };
        // let extension_bytes = self.extensions.as_ref().unwrap().to_bytes();
        let checksum_bytes = self.checksum.to_vec();

        [index_header_bytes, index_entry_bytes, extension_bytes, checksum_bytes].concat()
    }

    pub fn write_tree() {
        // We create an index from the file
        let mut index_path = std::env::current_dir().unwrap();
        index_path.push(".warp");
        index_path.push("index");

        // Read the index to a buffer
        let mut buffer = Vec::new();
        let _ = fs::File::open(&index_path).unwrap().read_to_end(&mut buffer);

        // Create a WarpIndex from the index fd
        let warp_index = WarpIndex::try_from(&mut Cursor::new(buffer.as_slice())).unwrap();


        // Using the IndexEntry we create the extensions.
        println!("{:#?}", warp_index.entries);
    }

    pub fn update_index(paths: Vec<PathBuf>) -> Self {
        let index;
        if !index_file_exists() {
            index = create_new_index();
            let entry_count = paths.len();
            // Create our header
            let new_index_header = IndexHeader::new([68, 73, 82, 67], 2, entry_count as u32);
            let mut index_entries = Vec::new();
            for file in paths {
                let index_entry = IndexEntry::entry_from_file(file);
                index_entries.push(index_entry);
            }
            // let extension: Option<IndexExtension> = None;
            
            
            // create a Sha1 object
            let mut hasher = Sha1::new();
            // process input message
            
            let mut bytes = Vec::new();
            bytes.extend(new_index_header.to_bytes());
            index_entries.iter().for_each(|entry| bytes.extend(entry.to_bytes()));
            // Note the extension here is not needed
            hasher.update(bytes);
            let mut checksum = [0u8; 20];

            let bytes = hasher.finalize();
            checksum.copy_from_slice(bytes.as_slice());

            WarpIndex {
                header: new_index_header,
                entries: index_entries,
                extensions: None,
                checksum,
            }


            // Create the IndexEntries from the paths
            // IndexEntry
        } else {
            // Get the current index binary file
            let mut index_path = std::env::current_dir().unwrap();
            index_path.push(".warp");
            index_path.push("index");

            // Open it.
            let mut index = fs::File::open(&index_path).unwrap();

            // Read it into this buffer
            let mut buffer = Vec::new();
            let _ = index.read_to_end(&mut buffer).unwrap();

            // Create a WarpIndex from the index file.
            let warp_index = WarpIndex::try_from(&mut Cursor::new(buffer.as_slice())).unwrap();

            // Create entries from the the paths passed in the function
            let mut index_entries = Vec::new();
            for file in paths {
                let index_entry = IndexEntry::entry_from_file(file);
                index_entries.push(index_entry);
            }

            // Extend with the one from the index file. Sort before writing once again.
            index_entries.extend_from_slice(&warp_index.entries);
            index_entries.sort_by(|a, b| a.path.cmp(&b.path));

            let new_warp_index = WarpIndex::without_extension(index_entries);
            // Write the bytes of this WarpIndex to the index file, we convert it to bytes format
            fs::OpenOptions::new().write(true).open(&index_path).unwrap().write_all(&new_warp_index.to_bytes()).unwrap();
            new_warp_index
        }

    }
}

impl Debug for WarpIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WarpIndex")
         .field("header", &self.header)
         .field("entries", &self.entries)
         .field("extensions", &self.extensions)
         .field("checksum", &self.checksum.iter().map(|ch| format!("{:02x}", ch)).collect::<String>())
         .finish()
    }
}

impl TryFrom<&mut Cursor<&[u8]>> for WarpIndex {
    type Error = IndexParseError;

    fn try_from(reader: &mut Cursor<&[u8]>) -> Result<Self, Self::Error> {
        let header = IndexHeader::try_from(&mut *reader).unwrap();
        let mut entries: Vec<IndexEntry> = Vec::new();
        for _ in 0..header.entry_count {

            entries.push(IndexEntry::try_from(&mut *reader).unwrap());
        }
        let mut signature = [0u8; 4];
        let extensions;
        reader.read_exact(&mut signature);
        if signature == [84, 82, 69, 69] {
            reader.set_position(reader.position() - 4);
            extensions = Some(IndexExtension::try_from(&mut *reader).unwrap());
        } else {
            reader.set_position(reader.position() - 4);
            extensions = None;
        }
        // Read the next four bytes here. If it is the signature tree, we return some, 
        // otherwise extensions is none.

        let mut checksum = [0u8; 20];
        reader.read_exact(&mut checksum);

        Ok(WarpIndex {
            header,
            entries,
            extensions,
            checksum
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_header_bytes() {
        let mut reader = Cursor::new(INDEX_DATA);
        let test_header = IndexHeader::try_from(&mut reader).unwrap();
        let header = IndexHeader::new([68, 73, 82, 67], 2, 5);

        assert_eq!(test_header, header);
    }

    #[test]
    fn test_index_entries() {
        let mut reader = Cursor::new(INDEX_DATA);
        let _ = IndexHeader::try_from(&mut reader).unwrap();
        let test_entry = IndexEntry::try_from(&mut reader).unwrap();

        assert_eq!(test_entry.path, "filea.txt".to_string());
        assert_eq!(test_entry.sha.iter().map(|ch| format!("{:02x}", ch)).collect::<String>(), "77ef3bbc6c333c6088eba7a7b0c4c26203ed9765".to_string());
    }

    #[test]
    fn test_warp_index() {
        let mut reader =  Cursor::new(INDEX_DATA);
        let warp_index = WarpIndex::try_from(&mut reader).unwrap();

        assert_eq!(warp_index.checksum, hex!("53b5bfc28367bc8aaf62e76f76ffa5567fcd2fec"));
    }

    #[test]
    fn test_warp_to_bytes() {
        let mut reader = Cursor::new(INDEX_DATA);
        let warp_index = WarpIndex::try_from(&mut reader).unwrap();

        let warp_bytes = warp_index.to_bytes();

        assert_eq!(warp_bytes, INDEX_DATA);
    }
}