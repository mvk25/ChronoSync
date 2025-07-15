use std::{env, fs, io::Write};

use chrono::{DateTime, Local};
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};


const AUTHOR: &'static str = "dummy author";
const COMMITTER: &'static str = "dummy author";
const AUTHOR_EMAIL: &'static str = "dummy@email.com";
const COMMITTER_EMAIL: &'static str = "dummy@email.com";

pub struct Commit<'a> {
    tree: String,
    parent_tree: Option<String>,
    author_name: &'a str,
    committer_name: &'a str,
    author_email: &'a str,
    commit_email: &'a str,
    commit_date: DateTime<Local>,
    commit_message: String
}

impl<'a> Commit<'a> {
    pub fn new(tree: String, parents: Option<String>, message: String) -> Self {        
        Commit {
            tree,
            parent_tree: parents,
            author_name: AUTHOR,
            committer_name: COMMITTER,
            author_email: AUTHOR_EMAIL,
            commit_email: COMMITTER_EMAIL,
            commit_date: Local::now(),
            commit_message: message,
        }
    }

    fn hash_content(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let tree_object_hash = format!("tree {}\n", self.tree);
        let parent_object_hash = if let Some(parent) = &self.parent_tree {
            format!("parent {}\n", parent)
        } else { String::new() };
        let author = format!("author {} <{}> {} {}\n", self.author_name, self.author_email, self.commit_date.timestamp(), self.commit_date.offset());
        let committer = format!("committer {} <{}> {} {}\n", self.committer_name, self.commit_email, self.commit_date.timestamp(), self.commit_date.offset());

        buf.extend_from_slice(tree_object_hash.as_bytes());
        buf.extend_from_slice(parent_object_hash.as_bytes());
        buf.extend_from_slice(author.as_bytes());
        buf.extend_from_slice(committer.as_bytes());
        buf.extend_from_slice(b"\n");
        buf.extend_from_slice(self.commit_message.as_bytes());
        buf.extend_from_slice(b"\n");

        buf
    }

    fn hash_commit_object(&self) -> Vec<u8> {
        let mut hashable = self.hash_content();
        let commit_header = format!("commit {}", hashable.len());

        [commit_header.as_bytes(), &hashable].concat()
    }

    pub fn hash(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut hasher = Sha1::new();
        let hash_object = self.hash_commit_object();

        hasher.update(hash_object);

        let result = hasher.finalize();

        Ok(hex::encode(result))
    }

    pub fn compress_to_object(&self) {
        let hash = self.hash_commit_object();

        let mut encoded = ZlibEncoder::new(Vec::new(), Compression::default());
        encoded.write_all(&hash).expect("Error writing to compressing stream");
        let compressed_bytes = encoded.finish().expect("Error compressing hash content");

        let object = self.hash().expect("Error hashing file");

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