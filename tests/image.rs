// SPDX-FileCopyrightText: 2016 kennytm
// SPDX-FileCopyrightText: 2019 Jasper Bryant-Greene
// SPDX-FileCopyrightText: 2025 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(feature = "image")]

use qrcode2::{
    EcLevel, QrCode, Version,
    image::{Luma, Rgb},
};

#[test]
fn test_annex_i_qr_as_image() {
    let code = QrCode::new(b"01234567").unwrap();
    let image = code.render::<Luma<u8>>().build();
    let expected = image::load_from_memory(include_bytes!("data/test_annex_i_qr_as_image.png"))
        .unwrap()
        .into_luma8();
    assert_eq!(image.dimensions(), expected.dimensions());
    assert_eq!(image.into_raw(), expected.into_raw());
}

#[test]
fn test_annex_i_micro_qr_as_image() {
    let code = QrCode::new_micro(b"01234567").unwrap();
    let image = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(Rgb([128, 0, 0]))
        .light_color(Rgb([255, 255, 128]))
        .build();
    let expected =
        image::load_from_memory(include_bytes!("data/test_annex_i_micro_qr_as_image.png"))
            .unwrap()
            .into_rgb8();
    assert_eq!(image.dimensions(), expected.dimensions());
    assert_eq!(image.into_raw(), expected.into_raw());
}

#[test]
fn test_annex_i_rmqr_as_image() {
    let code = QrCode::new_rect_micro(b"01234567").unwrap();
    let image = code.render::<Luma<u8>>().build();
    let expected = image::load_from_memory(include_bytes!("data/test_annex_i_rmqr_as_image.png"))
        .unwrap()
        .into_luma8();
    assert_eq!(image.dimensions(), expected.dimensions());
    assert_eq!(image.into_raw(), expected.into_raw());
}

#[test]
fn test_qr_v40_ec_h_as_image() {
    let code = QrCode::with_version(b"01234567", Version::Normal(40), EcLevel::H).unwrap();
    let image = code.render::<Luma<u8>>().build();
    let expected = image::load_from_memory(include_bytes!("data/test_qr_v40_ec_h_as_image.png"))
        .unwrap()
        .into_luma8();
    assert_eq!(image.dimensions(), expected.dimensions());
    assert_eq!(image.into_raw(), expected.into_raw());
}

#[test]
fn test_micro_qr_v4_ec_q_as_image() {
    let code = QrCode::with_version(b"01234567", Version::Micro(4), EcLevel::Q).unwrap();
    let image = code.render::<Luma<u8>>().build();
    let expected =
        image::load_from_memory(include_bytes!("data/test_micro_qr_v4_ec_q_as_image.png"))
            .unwrap()
            .into_luma8();
    assert_eq!(image.dimensions(), expected.dimensions());
    assert_eq!(image.into_raw(), expected.into_raw());
}

#[test]
fn test_rmqr_vr17x139_ec_h_as_image() {
    let code = QrCode::with_version(b"01234567", Version::RectMicro(17, 139), EcLevel::H).unwrap();
    let image = code.render::<Luma<u8>>().build();
    let expected =
        image::load_from_memory(include_bytes!("data/test_rmqr_vr17x139_ec_h_as_image.png"))
            .unwrap()
            .into_luma8();
    assert_eq!(image.dimensions(), expected.dimensions());
    assert_eq!(image.into_raw(), expected.into_raw());
}
