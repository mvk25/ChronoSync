use std::env;
use std::io::Write;
use std::{ffi::CString, fs, io::Read, path::PathBuf};

use sha1::{Digest, Sha1};
use flate2::Compression;
use flate2::write::ZlibEncoder;


#[derive(Clone)]
pub struct Blob {
    filename: PathBuf
}

impl Blob {
    // create an instance of a blob file
    pub fn new(filename: PathBuf) -> Self {
        Self {
            filename
        }
    }

    fn hash_content(&self) -> Vec<u8> {
        let mut file = fs::File::open(&self.filename).unwrap();

        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        let blob = buf.as_bytes();
        let blob_len = blob.len();

        let header = format!("blob {}", blob_len);
        let header= CString::new(header).expect("CString failed");

        let header_bytes = header.as_bytes_with_nul();
        let hash_object = [header_bytes, blob].concat();

        hash_object
        
    }

    // Hash a file producing an object file.
    pub fn hash_object(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut hasher = Sha1::new();
        let hash_object = self.hash_content();
        
        hasher.update(hash_object);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    pub fn compress_to_object(&self) {
        // Get the blob contents
        let hash = self.hash_content();

        // Compress the blob content
        let mut encoded = ZlibEncoder::new(Vec::new(), Compression::default());
        encoded.write_all(&hash).expect("Error writing to compressing stream");
        let compressed_bytes = encoded.finish().expect("Error compressing hash content");
        
        // Create object Identifier from filename;
        let object = self.hash_object().expect("Error hashing file object");

        // Get the root_directory and create object tree 
        let mut root = env::current_dir().expect("Unable to get cwd");
        root.push(".warp");
        root.push("objects");
        let (dir_hash, file_hash) = object.split_at(2);
        crate::auxiliary::push_dir_with_file(root.clone(), dir_hash, file_hash);

        root.push(dir_hash);
        root.push(file_hash);

        let mut compressed_file = fs::File::create(root).expect("Unable to open file");
        compressed_file.write(&compressed_bytes).expect("Error writing to file");
    }
}