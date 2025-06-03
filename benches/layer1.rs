use criterion::{criterion_group, criterion_main, Criterion};
use did_key::*;

fn bench_resolve(c: &mut Criterion) {
    let did = "did:key:z6Mkk7yqnGF3YwTrLpqrW6PGsKci7dNqh1CjnvMbzrMerSeL";
    c.bench_function("resolve", |b| b.iter(|| resolve(did).unwrap()));
}

fn bench_generate(c: &mut Criterion) {
    c.bench_function("generate_ed25519", |b| b.iter(|| generate::<Ed25519KeyPair>(None)));
}

fn bench_sign_verify(c: &mut Criterion) {
    let key = generate::<Ed25519KeyPair>(None);
    let msg = b"message to be signed";
    c.bench_function("sign", |b| b.iter(|| key.sign(msg)));
    let sig = key.sign(msg);
    c.bench_function("verify", |b| b.iter(|| key.verify(msg, &sig).unwrap()));
}

fn bench_document(c: &mut Criterion) {
    let key = generate::<Ed25519KeyPair>(None);
    c.bench_function("did_document", |b| b.iter(|| key.get_did_document(Config::default())));
}

criterion_group!(benches, bench_resolve, bench_generate, bench_sign_verify, bench_document);
criterion_main!(benches);
