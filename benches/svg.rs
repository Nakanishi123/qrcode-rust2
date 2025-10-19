// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(feature = "svg")]
#![feature(test)]

extern crate test;

use qrcode2::{QrCode, render::svg::Color};
use test::Bencher;

#[bench]
fn render_normal(b: &mut Bencher) {
    let code = QrCode::new(b"01234567").unwrap();
    b.iter(|| code.render::<Color<'_>>().build());
}

#[bench]
fn render_micro(b: &mut Bencher) {
    let code = QrCode::new_micro(b"01234567").unwrap();
    b.iter(|| code.render::<Color<'_>>().build());
}

#[bench]
fn render_rmqr(b: &mut Bencher) {
    let code = QrCode::new_rect_micro(b"01234567").unwrap();
    b.iter(|| code.render::<Color<'_>>().build());
}
