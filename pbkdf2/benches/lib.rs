#![no_std]
#![feature(test)]
extern crate pbkdf2;
extern crate sha_1;
extern crate sha2;
extern crate hmac;

extern crate test;

use test::Bencher;
use pbkdf2::pbkdf2;
use hmac::Hmac;

#[bench]
pub fn pbkdf2_hmac_sha1_16384_20(bh: &mut Bencher) {
    let password = b"my secure password";
    let salt = b"salty salt";
    let mut buf = [0u8; 20];
    bh.iter(|| {
        pbkdf2::<Hmac<sha_1::Sha1>>(password, salt, 16384, &mut buf);
        test::black_box(&buf);
    });
}

#[bench]
pub fn pbkdf2_hmac_sha256_16384_20(bh: &mut Bencher) {
    let password = b"my secure password";
    let salt = b"salty salt";
    let mut buf = [0u8; 20];
    bh.iter(|| {
        pbkdf2::<Hmac<sha2::Sha256>>(password, salt, 16384, &mut buf);
        test::black_box(&buf);
    });
}

#[bench]
pub fn pbkdf2_hmac_sha512_16384_20(bh: &mut Bencher) {
    let password = b"my secure password";
    let salt = b"salty salt";
    let mut buf = [0u8; 20];
    bh.iter(|| {
        pbkdf2::<Hmac<sha2::Sha512>>(password, salt, 16384, &mut buf);
        test::black_box(&buf);
    });
}