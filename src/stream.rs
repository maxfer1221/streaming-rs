// use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicU16;
use std::fs::File;
use crate::utils;

struct Stream {
    id: String,
    chunks: Chunks,
    chunk_count: u16,
    read_ready: bool,
}

struct Chunks {
    write: Chunk,
    write_id: AtomicU16,
    read: Chunk,
    read_id: AtomicU16,
}

pub struct Chunk {
    id: u16,
    file: File,
    bytes: Vec<u8>,
}

impl Stream {
    fn new(video_id: String, chunk_count: u16) -> std::io::Result<Self> {
        let files = utils::create_files(video_id, chunk_count)?;
        let r_id = 0;
        let w_id = (chunk_count as f32 / 2.0).ceil();
        Stream {
            id: video_id,
            chunk_count: chunk_count,
            chunks: Chunks {
                read: Chunk {
                    id: w_id,
                    file: files[0],
                    bytes: Vec::u8::new(),
                },
                read_id: r_id,
                write: Chunk {
                    id: r_id,
                    file: files[1],
                    bytes: Vec::u8::new(),
                }
                write_id: w_id,
            },
            read_ready: false,
        }
    }

    fn next_files(&mut self) {
        let w = self.chunks.write;
        let r = self.chunks.read;
        
        w.id = (w.id + 1) % self.chunk_count;
        r.id = (r.id + 1) % self.chunk_count;
        
        w.file = utils::get_chunk_file([self.id, w.id])?;
        r.file = utils::get_chunk_file([self.id, r.id])?;
    }

    fn set_bytes(&mut self, bytes: Vec<u8>) {
        self.chunks.write.bytes = bytes;
    }

    fn get_bytes(&mut self) -> Vec<u8> {
        self.chunks.read.bytes
    }
}
