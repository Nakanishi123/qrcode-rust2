// SPDX-FileCopyrightText: 2017 kennytm
// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(feature = "svg")]

use qrcode2::{QrCode, render::svg::Color};

#[test]
fn test_annex_i_qr_as_svg() {
    let code = QrCode::new(b"01234567").unwrap();
    let image = code.render::<Color<'_>>().build();
    let expected = include_str!("data/test_annex_i_qr_as_svg.svg");
    assert_eq!(&image, expected);
}

#[test]
fn test_annex_i_micro_qr_as_svg() {
    let code = QrCode::new_micro(b"01234567").unwrap();
    let image = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(Color("#800000"))
        .light_color(Color("#ffff80"))
        .build();
    let expected = include_str!("data/test_annex_i_micro_qr_as_svg.svg");
    assert_eq!(&image, expected);
}

#[test]
fn test_annex_i_rmqr_as_svg() {
    let code = QrCode::new_rect_micro(b"01234567").unwrap();
    let image = code.render::<Color<'_>>().build();
    let expected = include_str!("data/test_annex_i_rmqr_as_svg.svg");
    assert_eq!(&image, expected);
}
