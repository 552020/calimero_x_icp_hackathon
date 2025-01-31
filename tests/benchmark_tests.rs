use criterion::{black_box, Criterion, criterion_group, criterion_main};
use my_blockchain::{Miner, Transaction};

fn benchmark_mining(c: &mut Criterion) {
    let mut group = c.benchmark_group("mining");
    
    // Benchmark empty block mining
    group.bench_function("mine_empty_block", |b| {
        b.iter(|| {
            let miner = Miner::new();
            black_box(miner.mine_block())
        })
    });
    
    // Benchmark mining with transactions
    group.bench_function("mine_block_with_transactions", |b| {
        let transactions = vec![Transaction::new(); 10];
        b.iter(|| {
            let miner = Miner::new();
            black_box(miner.mine_block_with_transactions(transactions.clone()))
        })
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_mining);
criterion_main!(benches);
