use criterion::{criterion_group, criterion_main, Criterion};
use hex::decode;
use vrf::openssl::{CipherSuite, ECVRF};
use vrf::VRF;

fn bench_derive_public_key(c: &mut Criterion) {
    let mut vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
    let secret_key = decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721").unwrap();
    c.bench_function("derive_public_key", |b| {
        b.iter(|| vrf.derive_public_key(&secret_key).unwrap())
    });
}

fn bench_prove(c: &mut Criterion) {
    let mut vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
    let secret_key = decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721").unwrap();
    let alpha = b"sample".to_vec();
    c.bench_function("prove", |b| {
        b.iter(|| vrf.prove(&secret_key, &alpha).unwrap())
    });
}

fn bench_verify(c: &mut Criterion) {
    let mut vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
    let secret_key = decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721").unwrap();
    let public_key = vrf.derive_public_key(&secret_key).unwrap();
    let alpha = b"sample".to_vec();
    let proof = vrf.prove(&secret_key, &alpha).unwrap();
    c.bench_function("verify", |b| {
        b.iter(|| vrf.verify(&public_key, &proof, &alpha).unwrap())
    });
}

fn bench_proof_to_hash(c: &mut Criterion) {
    let mut vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
    let secret_key = decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721").unwrap();
    let alpha = b"sample".to_vec();
    let proof = vrf.prove(&secret_key, &alpha).unwrap();
    c.bench_function("proof_to_hash", |b| {
        b.iter(|| vrf.proof_to_hash(&proof).unwrap())
    });
}

criterion_group!(benches, bench_derive_public_key, bench_prove, bench_verify, bench_proof_to_hash);
criterion_main!(benches);
