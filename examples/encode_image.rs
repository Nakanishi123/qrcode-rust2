// SPDX-FileCopyrightText: 2017 kennytm
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use image::Luma;
use qrcode::QrCode;

fn main() {
    // Encode some data into bits.
    let code = QrCode::new(b"01234567").unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("/tmp/qrcode.png").unwrap();
}
