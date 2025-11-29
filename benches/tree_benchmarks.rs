use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use lattice::trees::binary_tree::BinaryTree;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn generate_shuffled_data(size: i32) -> Vec<i32> {
    let mut data: Vec<i32> = (0..size).collect();
    data.shuffle(&mut thread_rng());
    data
}

fn get_test_sizes() -> Vec<i32> {
    let mut sizes = vec![100];
    let mut current = 500;
    while current <= 10000 {
        sizes.push(current);
        current += 500;
    }
    sizes
}

fn bench_bst_insert_sequential(c: &mut Criterion) {
    let mut group = c.benchmark_group("BST/insert_sequential");
    
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut tree = BinaryTree::new();
                for i in 0..size {
                    tree.insert(black_box(i));
                }
            });
        });
    }
    
    group.finish();
}

fn bench_bst_insert_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("BST/insert_random");
    
    for size in [100, 1000, 10000].iter() {
        let data = generate_shuffled_data(*size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| {
                let mut tree = BinaryTree::new();
                for &val in data {
                    tree.insert(black_box(val));
                }
            });
        });
    }
    
    group.finish();
}

fn bench_bst_search_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("BST/search_random");
    
    for size in get_test_sizes().iter() {
        let data = generate_shuffled_data(*size);
        let mut tree = BinaryTree::new();
        for &val in &data {
            tree.insert(val);
        }
        
        let search_values: Vec<i32> = (0..*size).step_by(*size as usize / 10).collect();
        
        group.bench_with_input(BenchmarkId::from_parameter(size), &search_values, |b, vals| {
            let mut idx = 0;
            b.iter(|| {
                let val = vals[idx % vals.len()];
                idx += 1;
                tree.search(black_box(val))
            });
        });
    }
    
    group.finish();
}

fn bench_bst_search_sequential(c: &mut Criterion) {
    let mut group = c.benchmark_group("BST/search_sequential");
    
    for size in [100, 1000, 10000].iter() {
        let mut tree = BinaryTree::new();
        for i in 0..*size {
            tree.insert(i);
        }
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                tree.search(black_box(size - 1))
            });
        });
    }
    
    group.finish();
}

fn bench_bst_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("BST/delete");
    
    for size in [100, 1000, 10000].iter() {
        let data = generate_shuffled_data(*size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter_batched(
                || {
                    let mut tree = BinaryTree::new();
                    for &val in data {
                        tree.insert(val);
                    }
                    tree
                },
                |mut tree| {
                    tree.delete(black_box(data[data.len() / 2]))
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    
    group.finish();
}

fn bench_bst_height(c: &mut Criterion) {
    let mut group = c.benchmark_group("BST/height");
    
    for size in [100, 1000, 10000].iter() {
        let data = generate_shuffled_data(*size);
        let mut tree = BinaryTree::new();
        for &val in &data {
            tree.insert(val);
        }
        
        group.bench_with_input(BenchmarkId::from_parameter(size), &tree, |b, tree| {
            b.iter(|| tree.height());
        });
    }
    
    group.finish();
}

fn bench_bst_count_nodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("BST/count_nodes");
    
    for size in [100, 1000, 10000].iter() {
        let data = generate_shuffled_data(*size);
        let mut tree = BinaryTree::new();
        for &val in &data {
            tree.insert(val);
        }
        
        group.bench_with_input(BenchmarkId::from_parameter(size), &tree, |b, tree| {
            b.iter(|| tree.count_nodes());
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_bst_insert_sequential,
    bench_bst_insert_random,
    bench_bst_search_random,
    bench_bst_search_sequential,
    bench_bst_delete,
    bench_bst_height,
    bench_bst_count_nodes
);
criterion_main!(benches);
