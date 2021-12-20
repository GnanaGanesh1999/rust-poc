pub mod args;
pub mod read;
pub mod stats;
pub mod write;

const KILOBYTE: usize = 1024;
const CHUNK_SIZE: usize = 16 * KILOBYTE;
