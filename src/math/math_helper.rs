pub fn bary_centric(one : f32, two : f32, three : f32, amount1 : f32, amount2 : f32) -> f32 {
    one + ( two - three ) * amount1 + ( three - one ) * amount2
}

pub fn catmull_rom(one : f32, two : f32, three : f32, four : f32, amount : f32) -> f32 {
    let amount_squared = amount * amount;
    let amount_cubed   = amount_squared * amount;

    0.5 *
        ( 2.0 * two +
          ( three - one ) * amount +
          ( 2.0 * one - 5.0 * two + 4.0 * three - four ) * amount_squared +
          ( 3.0 * two - one - 3.0 * three + four )       * amount_cubed )
}

pub fn clamp(value : f32, min : f32, max : f32) -> f32 {
    let ret = if value > max { max } else { value };

    if ret < min { min } else { ret }
}


pub fn hermite(v1 : f32, tangent1 : f32, v2 : f32, tangent2 : f32, amount : f32) -> f32 {
    let one = v1 as f64;
    let two = v2 as f64;

    let tone = tangent1 as f64;
    let ttwo = tangent2 as f64;

    let s = amount as f64;

    let cubed   = s * s * s;
    let squared = s * s;

    if amount == 0.0 {
        one as f32
    } else if amount == 1.0 {
        two as f32
    } else {
        (
            ( 2.0 * one - 2.0 * two + ttwo + tone ) * cubed +
                ( 3.0 * two - 3.0 * one - 2.0 * tone - ttwo ) * squared +
                tone * s + one
        ) as f32
    }
}

pub fn lerp(v1 : f32, v2 : f32, amount : f32) -> f32 {
    v1 + ( v2 - v1 ) * amount
}

pub fn lerp_precise(v1 : f32, v2 : f32, amount : f32) -> f32 {
    ( ( 1.0 - amount ) * v1 ) + ( v2 * amount )
}
