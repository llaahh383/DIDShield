use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../src/system_setup.rs"]
mod system_setup;
use system_setup::{hash_to_point, random_scalar, scalar_from_hash};

fn bench_random_scalar(c: &mut Criterion) {
    c.bench_function("random_scalar", |b| b.iter(|| random_scalar()));
}

fn bench_scalar_from_hash(c: &mut Criterion) {
    let data = b"benchmark";
    c.bench_function("scalar_from_hash", |b| b.iter(|| scalar_from_hash(data)));
}

fn bench_hash_to_point(c: &mut Criterion) {
    let msg = b"message";
    c.bench_function("hash_to_point", |b| b.iter(|| hash_to_point(msg)));
}

fn bench_vrf_keygen(c: &mut Criterion) {
    c.bench_function("vrf_keygen", |b| b.iter(|| system_setup::vrf::keygen()));
}

fn bench_vrf_prove(c: &mut Criterion) {
    let keypair = system_setup::vrf::keygen();
    let msg = b"msg";
    c.bench_function("vrf_prove", |b| {
        b.iter(|| system_setup::vrf::prove(&keypair.sk, msg))
    });
}

fn bench_pedersen_setup(c: &mut Criterion) {
    c.bench_function("pedersen_setup", |b| b.iter(|| system_setup::pedersen::setup(5)));
}

fn bench_credential_h_agg(c: &mut Criterion) {
    let (issuers, _) = system_setup::credential::setup(3);
    let pks: Vec<_> = issuers.iter().map(|i| i.pk).collect();
    let pk_j = pks[0];
    c.bench_function("credential_h_agg", |b| b.iter(|| system_setup::credential::h_agg(&pks, &pk_j)));
}

fn bench_credential_setup(c: &mut Criterion) {
    c.bench_function("credential_setup", |b| b.iter(|| system_setup::credential::setup(3)));
}

fn bench_identity_setup(c: &mut Criterion) {
    let user = "user";
    c.bench_function("identity_setup", |b| b.iter(|| system_setup::identity::setup(user)));
}

fn bench_system_setup(c: &mut Criterion) {
    c.bench_function("system_setup", |b| b.iter(|| system_setup::system_setup(128, 3)));
}

criterion_group!(benches,
    bench_random_scalar,
    bench_scalar_from_hash,
    bench_hash_to_point,
    bench_vrf_keygen,
    bench_vrf_prove,
    bench_pedersen_setup,
    bench_credential_h_agg,
    bench_credential_setup,
    bench_identity_setup,
    bench_system_setup,
);
criterion_main!(benches);

