

pub fn perlin(x: f32, y: f32, p :&[i32; 512]) -> f32 {
    let xi = x as i32 & 255;
    let yi = y as i32 & 255;

    let xf = x - (x as i32) as f32;
    let yf = y - (y as i32) as f32;

    let u = fade(xf);
    let w = fade(yf);

    let ll = p[(p[xi as usize] + yi) as usize];
    let lt = p[(p[xi as usize] + yi + 1) as usize];
    let rr = p[(p[(xi + 1) as usize] + yi) as usize];
    let rt = p[(p[(xi + 1) as usize] + yi + 1) as usize];

    let x1 = lerp(grad(ll, xf, yf), grad(rr, xf - 1.0, yf), u);

    let x2 = lerp(grad(lt, xf, yf - 1.0), grad(rt, xf - 1.0, yf - 1.0), u);

    let result = lerp(x1, x2, w);

    return (result + 1f32) * 0.7f32;
}

fn grad(hash: i32, x: f32, y: f32) -> f32 {
    match hash & 3 {
        0 =>  x + y,
        1 => -x + y,
        2 =>  x - y,
        _ => -x - y,
    }
}

fn fade(t: f32) -> f32 {
    return t * t * t * (t * (t * 6f32 - 15f32) + 10f32);
}

fn lerp(a: f32, b: f32, x: f32) -> f32 {
    return a + x * (b - a);
}
