use rust_blockchain::block::Block;

fn main() {
    println!("Genesis block:");
    let mut block = Block::default();

    println!("{:#?}", &block);
}
