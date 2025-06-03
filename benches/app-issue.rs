use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/app_credential.rs"]
mod app_credential;
#[path = "../src/system_setup.rs"]
mod system_setup;

use app_credential::{user_register_request, vrfy_register_request, nextgen_auth_setup, app_credential as get_app_credential, verify_cred, nextgen_vrfy_auth_request};
use system_setup::random_scalar;

fn bench_user_register_request(c: &mut Criterion) {
    c.bench_function("user_register_request", |b| {
        b.iter(|| user_register_request(black_box(1)))
    });
}

fn bench_vrfy_register_request(c: &mut Criterion) {
    c.bench_function("vrfy_register_request", |b| {
        b.iter(|| {
            let req = user_register_request(1);
            vrfy_register_request(black_box(&req), black_box(1))
        })
    });
}

fn bench_nextgen_auth_setup(c: &mut Criterion) {
    c.bench_function("nextgen_auth_setup", |b| b.iter(|| nextgen_auth_setup()));
}

fn bench_app_credential(c: &mut Criterion) {
    c.bench_function("app_credential", |b| {
        b.iter(|| {
            let k_e = nextgen_auth_setup();
            let a_i = vec![random_scalar(), random_scalar()];
            let s_i = vec![vec![1u8,2,3], vec![4,5,6]];
            let prv_u = (random_scalar(), random_scalar());
            let l = 42u64;
            let phi = b"dummy phi";
            let A_l = a_i.clone();
            let cnt = 1u64;
            let ctx = b"context";
            get_app_credential(a_i, s_i, prv_u, l, phi, A_l, cnt, &k_e, ctx)
        })
    });
}

fn bench_verify_cred(c: &mut Criterion) {
    c.bench_function("verify_cred", |b| {
        b.iter(|| {
            let k_e = nextgen_auth_setup();
            let a_i = vec![random_scalar(), random_scalar()];
            let s_i = vec![vec![1u8,2,3], vec![4,5,6]];
            let prv_u = (random_scalar(), random_scalar());
            let l = 42u64;
            let phi = b"dummy phi";
            let A_l = a_i.clone();
            let cnt = 1u64;
            let ctx = b"context";
            let app = get_app_credential(a_i, s_i, prv_u, l, phi, A_l, cnt, &k_e, ctx);
            let mut t_app = Vec::new();
            verify_cred(&app.cred, l, phi, &app.tg, &mut t_app, &app.auth_req)
        })
    });
}

fn bench_nextgen_vrfy_auth_request(c: &mut Criterion) {
    c.bench_function("nextgen_vrfy_auth_request", |b| {
        b.iter(|| {
            let k_e = nextgen_auth_setup();
            let a_i = vec![random_scalar(), random_scalar()];
            let s_i = vec![vec![1u8,2,3], vec![4,5,6]];
            let prv_u = (random_scalar(), random_scalar());
            let l = 42u64;
            let phi = b"dummy phi";
            let A_l = a_i.clone();
            let cnt = 1u64;
            let ctx = b"context";
            let app = get_app_credential(a_i, s_i, prv_u, l, phi, A_l, cnt, &k_e, ctx);
            nextgen_vrfy_auth_request(&app.auth_req.inclusion_req)
        })
    });
}

criterion_group!(benches,
    bench_user_register_request,
    bench_vrfy_register_request,
    bench_nextgen_auth_setup,
    bench_app_credential,
    bench_verify_cred,
    bench_nextgen_vrfy_auth_request,
);
criterion_main!(benches);

