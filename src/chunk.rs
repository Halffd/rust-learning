#[derive(Debug, Clone)]
pub enum ChunkType {
    Modification(Chunk),
    Unchanged(Chunk),
}

impl From<ChunkType> for Chunk {
    fn from(chunk_wrapper: ChunkType) -> Chunk {
        match chunk_wrapper {
            ChunkType::Modification(chunk) | ChunkType::Unchanged(chunk) => chunk,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Chunk {
    pub offset: u16,
    pub ctype: InstructionType, // Assuming InstructionType is defined elsewhere
    pub opcode: u16,
    pub data: Option<BlockStorage>, // Assuming BlockStorage is defined elsewhere
    pub ref_data: Option<BlockStorage>,
    pub path: Vec<String>,
    pub ref_simple: Option<u16>,
    pub segment_idx: Option<u8>,
}

#[derive(Clone, Debug)]
pub struct Leaf {
    pub chunks: Vec<ChunkType>, // Assuming chunks are of type ChunkType
}

#[derive(Clone, Debug)]
pub struct Table {
    pub id: u32, // Adjust as necessary
}

// Function to decode integer from byte array
fn get_int(data: &[u8]) -> u32 {
    let result = data
        .iter()
        .fold(0u32, |acc, &byte| (acc << 8) | byte as u32);
    result
}

// Corruption site
fn process_chunks(leaf: &Leaf, leaf_buffer: &[u8], table: &Table) {
    let path = vec![
        "3".to_string(),
        "16".to_string(),
        "1".to_string(),
        "1".to_string(),
    ];
    let ui_chunk = leaf.chunks.iter().position(|wrapper| {
        println!("Gets here.");
        let chunk = Chunk::from(wrapper.clone());
        println!("after unwrapping.");

        if chunk.path == path {
            let mut n: Vec<u8> = vec![];
            if chunk.ref_simple.is_some_and(|r| r == 16) {
                if let Some(data) = chunk.data {
                    n = data.lookup_from_buffer(leaf_buffer).unwrap(); // Assuming this function is defined
                    println!("n: {:?}", n);
                    let decoded = get_int(&n[n[0] as usize..]); // Ensure this indexing is safe
                    return decoded + 128 == table.id; // Assuming table.id is defined
                }
            }
        }
        false
    });

    if let Some(index) = ui_chunk {
        println!("Found chunk at index: {}", index);
    } else {
        println!("No matching chunk found.");
    }
}

// Main function
fn main() {
    // Create necessary instances for Leaf, Table, and leaf_buffer here.
    let leaf = Leaf { chunks: vec![] }; // Populate with actual chunks
    let leaf_buffer = &[]; // Provide actual buffer data
    let table = Table { id: 0 }; // Provide actual table data

    process_chunks(&leaf, leaf_buffer, &table);
}
