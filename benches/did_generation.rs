use criterion::{criterion_group, criterion_main, Criterion, black_box};
use rand::rngs::OsRng;
use rand::RngCore;
use did_key::{generate, Ed25519KeyPair, didcore::Config};

fn bench_did_creation(c: &mut Criterion) {
    c.bench_function("did_creation", |b| {
        b.iter(|| {
            // 1. Generate randomness r <- PRNG(λ)
            let mut seed = [0u8; 32];
            OsRng.fill_bytes(&mut seed);

            // 2. (skDID, pkDID) <- KeyGen(1λ; r)
            let key = generate::<Ed25519KeyPair>(Some(&seed));

            // 3. DID <- Method.Construct(pkDID)
            let did = format!("did:key:{}", key.fingerprint());

            // 4. Register DID Document <- { id: DID, verificationMethod: pkDID }
            let doc = key.get_did_document(Config::default());

            // 5. Store skDID securely; publish pkDID to registry
            let _secret = key.private_key_bytes();
            let _public = key.public_key_bytes();

            black_box((did, doc, _secret, _public));
        })
    });
}

criterion_group!(benches, bench_did_creation);
criterion_main!(benches);
