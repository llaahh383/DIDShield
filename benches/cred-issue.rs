use criterion::{black_box, criterion_group, criterion_main, Criterion};
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;

#[path = "../src/credential_issuance.rs"]
mod credential_issuance;
#[path = "../src/system_setup.rs"]
mod system_setup;

use credential_issuance::{token_request, token_issuance, token_prove, token_verify, IssuerInfo};
use system_setup::{random_scalar, scalar_from_hash};

fn bench_token_request(c: &mut Criterion) {
    let a = random_scalar();
    let y_s = random_scalar() * RISTRETTO_BASEPOINT_POINT;
    c.bench_function("token_request", |b| {
        b.iter(|| token_request(black_box(a), black_box(y_s)))
    });
}

fn setup_issuers(n: usize) -> Vec<IssuerInfo> {
    (0..n)
        .map(|_| {
            let sk = random_scalar();
            let pk = sk * RISTRETTO_BASEPOINT_POINT;
            let agg_scalar = scalar_from_hash(pk.compress().as_bytes());
            IssuerInfo { sk, pk, agg_scalar }
        })
        .collect()
}

fn bench_token_issuance(c: &mut Criterion) {
    let a = random_scalar();
    let y_s = random_scalar() * RISTRETTO_BASEPOINT_POINT;
    let (blind_req, rand_a) = token_request(a, y_s);
    let issuers = setup_issuers(3);
    c.bench_function("token_issuance", |b| {
        b.iter(|| token_issuance(black_box(&blind_req), black_box(&issuers), black_box(&rand_a)))
    });
}

fn bench_token_prove(c: &mut Criterion) {
    let a = random_scalar();
    let y_s = random_scalar() * RISTRETTO_BASEPOINT_POINT;
    let (blind_req, rand_a) = token_request(a, y_s);
    let issuers = setup_issuers(3);
    let (token, partial_sigs) = token_issuance(&blind_req, &issuers, &rand_a);
    c.bench_function("token_prove", |b| {
        b.iter(|| token_prove(black_box(&token), black_box(&partial_sigs), black_box(&rand_a)))
    });
}

fn bench_token_verify(c: &mut Criterion) {
    let a = random_scalar();
    let y_s = random_scalar() * RISTRETTO_BASEPOINT_POINT;
    let (blind_req, rand_a) = token_request(a, y_s);
    let issuers = setup_issuers(3);
    let (token, partial_sigs) = token_issuance(&blind_req, &issuers, &rand_a);
    let zk_proof = token_prove(&token, &partial_sigs, &rand_a);
    c.bench_function("token_verify", |b| {
        b.iter(|| token_verify(black_box(&token), black_box(&zk_proof), black_box(&blind_req), black_box(&issuers)))
    });
}

criterion_group!(benches, bench_token_request, bench_token_issuance, bench_token_prove, bench_token_verify);
criterion_main!(benches);

