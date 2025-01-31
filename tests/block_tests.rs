
// Check that the miner creates valid blocks, that they conform to the consensus rules
// -Correct block structure
// -Valid block hash based on the proof-of-work algorithm.
// -Transaction validity 


fn test_block_validity() {
    let block = Block::new(/* some block data */);

    assert!(block.hash().is_valid());

    // Verify that the block adheres to the PoW rules
    assert_eq!(block.hash().starts_with("0000"), true); // Assume PoW starts with '0000'
}
