/// Convert RGB to Lab
pub fn rgb_to_lab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let (x, y, z) = rgb_to_xyz(r, g, b);
    xyz_to_lab(x, y, z)
}

/// Convert RGB to XYZ
fn rgb_to_xyz(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let r = if r > 0.04045 {
        ((r + 0.055) / 1.055).powf(2.4)
    } else {
        r / 12.92
    };
    let g = if g > 0.04045 {
        ((g + 0.055) / 1.055).powf(2.4)
    } else {
        g / 12.92
    };
    let b = if b > 0.04045 {
        ((b + 0.055) / 1.055).powf(2.4)
    } else {
        b / 12.92
    };

    let r = r * 100.0;
    let g = g * 100.0;
    let b = b * 100.0;

    let x = r * 0.4124564 + g * 0.3575761 + b * 0.1804375;
    let y = r * 0.2126729 + g * 0.7151522 + b * 0.0721750;
    let z = r * 0.0193339 + g * 0.1191920 + b * 0.9503041;

    (x, y, z)
}

/// Convert XYZ to Lab
fn xyz_to_lab(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let x = x / 95.047;
    let y = y / 100.000;
    let z = z / 108.883;

    let x = if x > 0.008856 {
        x.powf(1.0 / 3.0)
    } else {
        (7.787 * x) + (16.0 / 116.0)
    };
    let y = if y > 0.008856 {
        y.powf(1.0 / 3.0)
    } else {
        (7.787 * y) + (16.0 / 116.0)
    };
    let z = if z > 0.008856 {
        z.powf(1.0 / 3.0)
    } else {
        (7.787 * z) + (16.0 / 116.0)
    };

    let l = (116.0 * y) - 16.0;
    let a = 500.0 * (x - y);
    let b = 200.0 * (y - z);

    (l, a, b)
}

/// Calculate Delta E
pub fn delta_e(lab1: (f32, f32, f32), lab2: (f32, f32, f32)) -> f32 {
    let (l1, a1, b1) = lab1;
    let (l2, a2, b2) = lab2;

    ((l1 - l2).powi(2) + (a1 - a2).powi(2) + (b1 - b2).powi(2)).sqrt()
}
