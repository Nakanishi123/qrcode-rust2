// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(feature = "image")]
#![feature(test)]

extern crate test;

use qrcode2::{QrCode, image::Luma};
use test::Bencher;

#[bench]
fn new(b: &mut Bencher) {
    b.iter(|| QrCode::new(b"01234567").unwrap());
}

#[bench]
fn new_micro(b: &mut Bencher) {
    b.iter(|| QrCode::new_micro(b"01234567").unwrap());
}

#[bench]
fn new_rect_micro(b: &mut Bencher) {
    b.iter(|| QrCode::new_rect_micro(b"01234567").unwrap());
}

#[bench]
fn render_normal(b: &mut Bencher) {
    let code = QrCode::new(b"01234567").unwrap();
    b.iter(|| code.render::<Luma<u8>>().build());
}

#[bench]
fn render_micro(b: &mut Bencher) {
    let code = QrCode::new_micro(b"01234567").unwrap();
    b.iter(|| code.render::<Luma<u8>>().build());
}

#[bench]
fn render_rmqr(b: &mut Bencher) {
    let code = QrCode::new_rect_micro(b"01234567").unwrap();
    b.iter(|| code.render::<Luma<u8>>().build());
}
