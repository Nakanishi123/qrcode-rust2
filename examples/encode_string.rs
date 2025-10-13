// SPDX-FileCopyrightText: 2017 kennytm
// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use qrcode::QrCode;

fn main() {
    let code = QrCode::new(b"Hello").unwrap();
    let string = code.render::<char>().quiet_zone(false).module_dimensions(2, 1).build();
    println!("{string}");
}
