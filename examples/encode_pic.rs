// SPDX-FileCopyrightText: 2024 Alexis Hildebrandt
// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use qrcode2::{QrCode, render::pic};

fn main() {
    let code = QrCode::new(b"01234567").unwrap();
    let image = code.render::<pic::Color>().min_dimensions(1, 1).build();
    println!("{image}");
}
