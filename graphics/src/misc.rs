
pub fn maybe<T>(option: Option<T>, s: &'static str) -> T
{
    assert!(option.is_some(), s);
    option.unwrap()
}


pub fn normalize_vec(v: (f32, f32, f32)) -> (f32, f32, f32)
{
    let norm = (v.0*v.0 + v.1*v.1 + v.2*v.2).sqrt();
    (v.0/norm, v.1/norm, v.2/norm)
}

pub fn v_prod(u: (f32, f32, f32), v: (f32, f32, f32)) -> (f32, f32, f32)
{
    (
        u.1*v.2 - u.2*v.1,
        -u.0*v.2 + u.2*v.0,
        u.0*v.1 - u.1*v.0
    )
}

