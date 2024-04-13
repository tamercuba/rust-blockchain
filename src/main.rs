use rust_blockchain::{ block::Block, hash::u64_bytes };

fn main() {
    println!("Genesis block:");
    let mut block = Block::new(
        1,
        1,
        vec![0; 32],
        "test".to_string(),
        0x00ffffffffffffffffffffffffffffff
    );

    println!("{:#?}", &block);
    println!("difficulty: {:#x}", block.difficulty);

    // block.set_hash();

    // println!("{:#?}", &block);
    // println!("difficulty: {:#x}", block.difficulty);

    let test = 257u64;
    println!("[Test normal u64: {:?}]", test);
    println!("[Test bytes u64 : {:?}]", u64_bytes(&test));
}
