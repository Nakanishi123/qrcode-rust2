// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(feature = "eps")]

use qrcode2::{EcLevel, QrCode, Version, render::eps::Color};

#[test]
fn test_annex_i_qr_as_eps() {
    let code = QrCode::new(b"01234567").unwrap();
    let image = code.render::<Color>().build();
    let expected = include_str!("data/test_annex_i_qr_as_eps.eps");
    assert_eq!(&image, expected);
}

#[test]
fn test_annex_i_micro_qr_as_eps() {
    let code = QrCode::with_version(b"01234567", Version::Micro(2), EcLevel::L).unwrap();
    let image = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(Color([0.5, 0.0, 0.0]))
        .light_color(Color([1.0, 1.0, 0.5]))
        .build();
    let expected = include_str!("data/test_annex_i_micro_qr_as_eps.eps");
    assert_eq!(&image, expected);
}

#[test]
fn test_annex_i_rmqr_as_eps() {
    let code = QrCode::with_version(b"01234567", Version::RectMicro(15, 43), EcLevel::M).unwrap();
    let image = code.render::<Color>().build();
    let expected = include_str!("data/test_annex_i_rmqr_as_eps.eps");
    assert_eq!(&image, expected);
}

#[test]
fn test_qr_v40_ec_h_as_eps() {
    let code = QrCode::with_version(b"01234567", Version::Normal(40), EcLevel::H).unwrap();
    let image = code.render::<Color>().build();
    let expected = include_str!("data/test_qr_v40_ec_h_as_eps.eps");
    assert_eq!(&image, expected);
}

#[test]
fn test_micro_qr_v4_ec_q_as_eps() {
    let code = QrCode::with_version(b"01234567", Version::Micro(4), EcLevel::Q).unwrap();
    let image = code.render::<Color>().build();
    let expected = include_str!("data/test_micro_qr_v4_ec_q_as_eps.eps");
    assert_eq!(&image, expected);
}

#[test]
fn test_rmqr_vr17x139_ec_h_as_eps() {
    let code = QrCode::with_version(b"01234567", Version::RectMicro(17, 139), EcLevel::H).unwrap();
    let image = code.render::<Color>().build();
    let expected = include_str!("data/test_rmqr_vr17x139_ec_h_as_eps.eps");
    assert_eq!(&image, expected);
}
