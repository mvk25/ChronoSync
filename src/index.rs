use core::{error, fmt};
use std::{fmt::Debug, io::{BufReader, Cursor, Read}, time::{Duration, SystemTime}};
use hex_literal::hex;
use chrono::{DateTime, Utc};

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
    // fn new(ctime_s: u32, ctime_n: u32, mtime: u32, mtime_s: u32, dev: u32, ino: u32, mode: u32, uid: u32, gid: u32, filesize: u32, sha: [u8; 20], flags: u16, path: String) -> Self {

    // }
}

const epoch: SystemTime = SystemTime::UNIX_EPOCH;
impl Debug for IndexEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndexEntry")
         .field("ctime_seconds", &DateTime::from_timestamp(self.ctime_seconds.into(), 0).unwrap())
         .field("ctime_nanoseconds", &DateTime::from_timestamp(self.ctime_seconds.into(), self.ctime_nanoseconds.into()).unwrap())
         .field("mtime_seconds", &DateTime::from_timestamp(self.mtime_seconds.into(), 0).unwrap())
         .field("mtime_nanoseconds", &DateTime::from_timestamp(self.mtime_seconds.into(), self.mtime_nanoseconds.into()).unwrap())
         .field("dev", &self.dev)
         .field("ino", &self.ino)
         .field("mode", &self.mode)
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
    type Error = String;

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

#[derive(Debug)]
pub enum IndexExtension {
    CacheTree(CacheTreeExtension),
    Unknown {
        signature: [u8; 4],
        data: Vec<u8>,
    },
}

#[derive(Debug)]
pub struct CacheTreeEntry {
    pub path: String,
    pub entry_count: u32,
    pub subtree_count: u32,
    pub sha: [u8; 20],
    pub subtrees: Vec<CacheTreeEntry>
}

#[derive(Debug)]
pub struct CacheTreeExtension {
    pub entries: Vec<CacheTreeEntry>
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

#[derive(Debug)]
pub struct WarpIndex {
    pub header: IndexHeader,
    pub entries: Vec<IndexEntry>,
    pub extensions: IndexExtension,
    pub checksum: [u8; 20]
}

// impl TryFrom<&mut Cursor<&[u8]>> for WarpIndex {
//     type Error = IndexParseError;

//     fn try_from(reader: &mut Cursor<&[u8]>) -> Result<Self, Self::Error> {
//         let header = IndexHeader::try_from(reader)?;
//         let entries = IndexEntry::try_from(reader)?;
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_warp_index() {
        let mut reader = Cursor::new(INDEX_DATA);
        let test_header = IndexHeader::new([68, 73, 82, 67], 2, 5);
        let byte_header = IndexHeader::try_from(&mut reader).unwrap();

        let entry_one = IndexEntry::try_from(&mut reader).unwrap();
        print!("{:?}", entry_one);

        assert_eq!(test_header, byte_header);
    }

    #[test]
    fn test_header_from_bytes() {
        // let test_header = IndexHeader::new([68, 73, 82, 67], 2, 5);
        // let byte_header = IndexHeader::try_from(INDEX_DATA).unwrap();

        // assert_eq!(test_header, byte_header);

    }

    const INDEX_DATA: &[u8] = &hex!(
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
}