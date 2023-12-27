//not trying to go crazy with this right now

pub fn dot(a: &[f64; 2], b: &[f64; 2]) -> f64 {
    a[0]*b[0] + a[1]*b[1]
}

pub fn length(v: &[f64; 2]) -> f64 {
    dot(v, v).sqrt()
}

pub fn normalized(v: &[f64; 2]) -> [f64; 2] {
    let len = length(v);    
    [ v[0] / len, v[1] / len]
}

pub fn angle_for_down_vector(v: &[f64; 2]) -> f64 {
    let v = normalized(v);
    let ang = v[0].atan2(v[1]);
    -ang
}