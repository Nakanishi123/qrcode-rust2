// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]

extern crate test;

use qrcode2::QrCode;
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
