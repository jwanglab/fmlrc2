
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::Cursor;

use fmlrc::bv_bwt::BitVectorBWT;
use fmlrc::bwt_converter::convert_to_vec;
use fmlrc::read_correction::bridge_kmers;
use fmlrc::ropebwt2_util::create_bwt_from_strings;
use fmlrc::string_util::*;

fn get_constant_bwt() -> BitVectorBWT {
    //build the dataset
    let const_string = "AACGGATCAAGCTTACCAGTATTTACGT";
    let rep_count = 30;
    let mut data: Vec<&str> = vec![];
    for _i in 0..rep_count {
        data.push(&const_string);
    }

    //build our BWT
    let bwt_string = create_bwt_from_strings(&data).unwrap();
    let bwt_cursor = Cursor::new(bwt_string);
    let vec = convert_to_vec(bwt_cursor);
    let mut bv_bwt: BitVectorBWT = BitVectorBWT::new();
    bv_bwt.load_vector(vec);
    bv_bwt
}

pub fn bench_string_util(c: &mut Criterion) {
    //this is the correct sequence, alter it below
    let query = convert_stoi(&"AACGGATCAAGCTTACCAGTATTTACGT");
    
    c.bench_function("convert_stoi", |b| b.iter(|| {
        black_box(convert_stoi(&"AACGGATCAAGCTTACCAGTATTTACGT"));
    }));

    c.bench_function("convert_itos", |b| b.iter(|| {
        black_box(convert_itos(&query));
    }));

    c.bench_function("reverse_complement_i", |b| b.iter(|| {
        black_box(reverse_complement_i(&query));
    }));
}

pub fn bench_count_kmer(c: &mut Criterion) {
    let bwt: BitVectorBWT = get_constant_bwt();

    //this is the correct sequence, alter it below
    let query = convert_stoi(&"AACGGATCAAGCTTACCAGTATTTACGT");
    let altered_query = convert_stoi(&"AACGGATCAAGCTTACCAGTATTTACGA");
    let mut counts: Vec<u64> = vec![0; 4];

    c.bench_function("count_kmer", |b| b.iter(|| {
        black_box(bwt.count_kmer(&query));
    }));

    c.bench_function("absent_count_kmer", |b| b.iter(|| {
        black_box(bwt.count_kmer(&altered_query));
    }));

    c.bench_function("prefix_kmer", |b| b.iter(|| {
        black_box(bwt.prefix_kmer_noalloc(&query[1..], &vec![1, 2, 3, 5], &mut counts));
    }));

    c.bench_function("absent_prefix_kmer", |b| b.iter(|| {
        black_box(bwt.prefix_kmer_noalloc(&altered_query[1..], &vec![1, 2, 3, 5], &mut counts));
    }));

    //okay, now test the actual bridging
    let seed = convert_stoi(&"AACGGAT");
    let target = convert_stoi(&"TTTACGT");
    let min_count = 15;
    let branch_lim = 20;
    let max_branch_len = 40;
    c.bench_function("bridge_kmers", |b| b.iter(|| {
        black_box(bridge_kmers(&bwt, &seed, &target, min_count, branch_lim, max_branch_len));
    }));
}

//criterion_group!(benches, bench_string_util);
criterion_group!(benches, bench_count_kmer);
criterion_main!(benches);