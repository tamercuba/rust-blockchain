use rust_blockchain::block::Block;

#[test]
fn test_genesis_block() {
    let block = Block::default();
    let zero_hash = vec![0; 32];
    assert_eq!(block.index, 0);
    assert_eq!(block.hash, zero_hash);
    assert_eq!(block.prev_block_hash, zero_hash);
    assert_eq!(block.transactions, "genesis!");
    assert_eq!(block.nonce, 0);
}

#[test]
fn test_new_block() {
    let block_index = 1;
    let block_timestamp = 1;
    let block_prev_block_hash = vec![1; 32];
    let block_transactions = "new block".to_string();
    let block_difficulty = 0x000fffffffffffffffffffffffffffff;
    let block = Block::new(
        block_index,
        block_timestamp,
        block_prev_block_hash.clone(),
        block_transactions.clone(),
        block_difficulty
    );

    assert_eq!(block.index, block_index);
    assert_eq!(block.timestamp, block_timestamp);
    assert_eq!(block.prev_block_hash, block_prev_block_hash);
    assert_eq!(block.transactions, block_transactions);
    assert_eq!(block.difficulty, block_difficulty);
}

#[test]
fn test_set_hash() {
    let mut block = Block::new(
        1,
        1,
        vec![0; 32],
        "test".to_string(),
        0x00ffffffffffffffffffffffffffffff
    );

    assert_eq!(block.index, 1);
    assert_eq!(block.timestamp, 1);
    assert_eq!(block.hash, vec![0;32]);
    assert_eq!(block.prev_block_hash, vec![0;32]);
    assert_eq!(block.transactions, "test");
    assert_eq!(block.nonce, 0);

    block.set_hash();

    let expected_hash = vec![
        56,
        36,
        187,
        157,
        107,
        60,
        18,
        85,
        249,
        74,
        17,
        209,
        17,
        53,
        123,
        61,
        54,
        234,
        254,
        170,
        197,
        21,
        89,
        82,
        192,
        91,
        175,
        128,
        230,
        250,
        67,
        0
    ];
    assert_eq!(block.hash, expected_hash);
    assert_ne!(block.nonce, 0);
}
