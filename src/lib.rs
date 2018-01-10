use std::mem;
use std::slice;
use std::os::raw::c_void;

const PERM: [usize; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

#[no_mangle]
pub fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub fn perlin_noise(
    ptr: *mut u8,
    width: usize,
    height: usize,
    time: f64,
    octaves: i32,
    persistence: f64,
    repeat_x: f64,
    repeat_y: f64,
    repeat_z: f64,
    size_x: f64,
    size_y: f64,
    size_z: f64,
) {
    let size = width * height * 4;
    let colors = unsafe { slice::from_raw_parts_mut(ptr, size) };
    perlin_noise_slice(
        colors,
        width,
        height,
        time,
        octaves,
        persistence,
        repeat_x,
        repeat_y,
        repeat_z,
        size_x,
        size_y,
        size_z,
    )
}

fn perlin_noise_slice(
    colors: &mut [u8],
    width: usize,
    height: usize,
    time: f64,
    octaves: i32,
    persistence: f64,
    repeat_x: f64,
    repeat_y: f64,
    repeat_z: f64,
    size_x: f64,
    size_y: f64,
    size_z: f64,
) {
    let mut i = 0;
    for y in 0..height {
        for x in 0..width {
            let p = octave_perlin(
                (x as f64) / (3.0 * size_x),
                (y as f64) / (3.0 * size_y),
                time / (3.0 * size_z),
                octaves,
                persistence,
                repeat_x / 3.0,
                repeat_y / 3.0,
                repeat_z / 3.0,
            );
            colors[i] = 0;
            colors[i + 1] = 0;
            colors[i + 2] = 0;
            colors[i + 3] = (p * 255.0) as u8;
            i += 4;
        }
    }
}

// #[test]
// fn test_perlin_noise_slice() {
//     let colors = &mut [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
//     perlin_noise_slice(colors, 2, 2, 3.0, 1, 1.0, 0);
//     println!(
//         "perlin_noise_slice: {}, {}, {}, {}",
//         colors[3], colors[7], colors[11], colors[15]
//     );
// }

fn octave_perlin(
    x: f64,
    y: f64,
    z: f64,
    octaves: i32,
    persistence: f64,
    repeat_x: f64,
    repeat_y: f64,
    repeat_z: f64,
) -> f64 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut max_value = 0.0;
    for _i in 0..octaves {
        total += perlin(
            x * frequency,
            y * frequency,
            z * frequency,
            repeat_x,
            repeat_y,
            repeat_z,
        ) * amplitude;
        max_value += amplitude;
        amplitude *= persistence;
        frequency *= 2.0;
    }
    return total / max_value;
}

// #[test]
// fn test_octave_perlin() {
//     let val = octave_perlin(1.11, 1.12, 0.0, 1, 1.0, 0);
//     println!("octave_perlin: {}", val);
//     assert_eq!("ok", "ok");
// }

fn perlin(mut x: f64, mut y: f64, mut z: f64, repeat_x: f64, repeat_y: f64, repeat_z: f64) -> f64 {
    println!("=============================================");
    if repeat_x > 0.0 {
        x %= repeat_x;
    }
    if repeat_y > 0.0 {
        y %= repeat_y;
    }
    if repeat_z > 0.0 {
        z %= repeat_z;
    }

    let xi = (x as usize) & 255;
    let yi = (y as usize) & 255;
    let zi = (z as usize) & 255;

    let xf = x - ((x as i32) as f64);
    let yf = y - ((y as i32) as f64);
    let zf = z - ((z as i32) as f64);
    println!(
        "x, y, z, xi, yi, zi, xf, yf, zf: {}, {}, {}, {}, {}, {}, {}, {}, {}",
        x, y, z, xi, yi, zi, xf, yf, zf
    );

    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    let xi1 = inc(xi, repeat_x as usize) % 256;
    let yi1 = inc(yi, repeat_y as usize) % 256;
    let zi1 = inc(zi, repeat_z as usize) % 256;

    let xiyi = PERM[(PERM[xi] + yi) % 256];
    let xiyi1 = PERM[(PERM[xi] + yi1) % 256];
    let xi1yi = PERM[(PERM[xi1] + yi) % 256];
    let xi1yi1 = PERM[(PERM[xi1] + yi1) % 256];

    let aaa = PERM[(xiyi + zi) % 256];
    let aba = PERM[(xiyi1 + zi) % 256];
    let aab = PERM[(xiyi + zi1) % 256];
    let abb = PERM[(xiyi1 + zi1) % 256];
    let baa = PERM[(xi1yi + zi) % 256];
    let bba = PERM[(xi1yi1 + zi) % 256];
    let bab = PERM[(xi1yi + zi1) % 256];
    let bbb = PERM[(xi1yi1 + zi1) % 256];

    println!(
        "aaa-bbb: {}, {}, {}, {}, {}, {}, {}, {}",
        aaa, aba, aab, abb, baa, bba, bab, bbb
    );

    let y1 = lerp(
        lerp(grad(aaa, xf, yf, zf), grad(baa, xf - 1.0, yf, zf), u),
        lerp(
            grad(aba, xf, yf - 1.0, zf),
            grad(bba, xf - 1.0, yf - 1.0, zf),
            u,
        ),
        v,
    );

    let y2 = lerp(
        lerp(
            grad(aab, xf, yf, zf - 1.0),
            grad(bab, xf - 1.0, yf, zf - 1.0),
            u,
        ),
        lerp(
            grad(abb, xf, yf - 1.0, zf - 1.0),
            grad(bbb, xf - 1.0, yf - 1.0, zf - 1.0),
            u,
        ),
        v,
    );

    println!(
        "y1, y2, w, lerped: {}, {}, {}, {}",
        y1,
        y2,
        w,
        lerp(y1, y2, w)
    );

    return (lerp(y1, y2, w) + 1.0) / 2.0;
}

// #[test]
// fn test_perlin() {
//     let val = perlin(0.1, 0.2, 0.3, 0);
//     println!("perlin: {}", val);
//     assert_eq!("ok", "ok");
// }

fn fade(t: f64) -> f64 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

fn inc(mut num: usize, repeat: usize) -> usize {
    num += 1;
    if repeat > 0 {
        num %= repeat;
    }
    return num as usize;
}

fn grad(hash: usize, x: f64, y: f64, z: f64) -> f64 {
    let h = hash & 15;
    println!("grad: {}, {}, {}, {}", h, x, y, z);
    let u = if h < 8 { x } else { y };
    let v = if h < 4 {
        y
    } else if h == 12 || h == 14 {
        x
    } else {
        z
    };
    return (if (h & 1) == 0 { u } else { -u }) + (if (h & 2) == 0 { v } else { -v });
}

fn lerp(a: f64, b: f64, x: f64) -> f64 {
    return a + x * (b - a);
}
