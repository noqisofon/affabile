use std::cmp::{PartialEq};
use std::ops::{Add,Sub,Div,Mul};

use super::super::math::math_helper;

pub struct Vector2f {
    pub x : f32,
    pub y : f32,
}

impl Vector2f {
    pub fn zero() -> Vector2f {
        Vector2f { x: 0.0, y: 0.0 }
    }

    pub fn one() -> Vector2f {
        Vector2f { x: 1.0, y: 1.0 }
    }

    pub fn unit_y() -> Vector2f {
        Vector2f { x: 0.0, y: 1.0 }
    }

    pub fn unit_x() -> Vector2f {
        Vector2f { x: 1.0, y: 0.0 }
    }

    fn debug_display_string(&self) -> String {
        format!( "{} {}", self.x, self.y )
    }

    pub fn new(x : f32, y : f32) -> Vector2f {
        Vector2f { x: x, y: y }
    }

    pub fn distance(v1 : &Vector2f, v2 : &Vector2f) -> f32 {
        let volume_x = v1.x - v2.x;
        let volume_y = v1.y - v2.y;

        ( ( volume_x * volume_x ) + ( volume_y * volume_y ) ).sqrt()
    }

    pub fn distance_squared(v1 : &Vector2f, v2 : &Vector2f) -> f32 {
        let volume_x = v1.x - v2.x;
        let volume_y = v1.y - v2.y;

        ( volume_x * volume_x ) + ( volume_y * volume_y )
    }

    pub fn dot(&self, other : &Vector2f) -> f32 {
        ( self.x * self.x ) + ( other.y * other.y )
    }

    pub fn hermite(v1 : &Vector2f, tangent1 : &Vector2f, v2 : &Vector2f, tangent2 : &Vector2f, amount : f32) -> Vector2f {
        Vector2f { x: math_helper::hermite( v1.x, tangent1.x, v2.x, tangent2.x, amount ),
                   y: math_helper::hermite( v1.y, tangent1.y, v2.y, tangent2.y, amount ) }
    }

    pub fn length(&self) -> f32 {
        ( ( self.x * self.x ) + ( self.y * self.y ) ).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        ( self.x * self.x ) + ( self.y * self.y )
    }

    pub fn max(v1 : &Vector2f, v2 : &Vector2f) -> Vector2f {
        let x = if v1.x > v2.x { v1.x } else { v2.x };
        let y = if v1.y > v2.y { v1.y } else { v2.y };

        Vector2f { x: x, y: y }
    }

    pub fn min(v1 : &Vector2f, v2 : &Vector2f) -> Vector2f {
        let x = if v1.x < v2.x { v1.x } else { v2.x };
        let y = if v1.y < v2.y { v1.y } else { v2.y };

        Vector2f { x: x, y: y }
    }

    pub fn bary_centric(v1 : &Vector2f, v2 : &Vector2f, v3 : &Vector2f, amount1 : f32, amount2 : f32) -> Vector2f {
        Vector2f { x: math_helper::bary_centric( v1.x, v2.x, v3.x, amount1, amount2 ),
                   y: math_helper::bary_centric( v1.y, v2.y, v3.y, amount1, amount2 ) }
    }

    pub fn catmull_rom(v1 : &Vector2f, v2 : &Vector2f, v3 : &Vector2f, amount1 : f32, amount2 : f32) -> Vector2f {
        Vector2f { x: math_helper::catmull_rom( v1.x, v2.x, v3.x, amount1, amount2 ),
                   y: math_helper::catmull_rom( v1.y, v2.y, v3.y, amount1, amount2 ) }
    }

    pub fn clamp(value : &Vector2f, min : &Vector2f, max : &Vector2f) -> Vector2f {
        Vector2f { x: math_helper::clamp( value.x, min.x, max.x ),
                   y: math_helper::clamp( value.y, min.y, max.y ) }
    }

    pub fn lerp(v1 : &Vector2f, v2 : &Vector2f, amount : f32) -> Vector2f {
        Vector2f { x: math_helper::lerp( v1.x, v2.x, amount ),
                   y: math_helper::lerp( v1.y, v2.y, amount ) }
    }

    pub fn lerp_precise(v1 : &Vector2f, v2 : &Vector2f, amount : f32) -> Vector2f {
        Vector2f { x: math_helper::lerp_precise( v1.x, v2.x, amount ),
                   y: math_helper::lerp_precise( v1.y, v2.y, amount ) }
    }
}

impl Add for Vector2f {
    type Output = Vector2f;

    fn add(self, other : Vector2f) -> Vector2f {
        Vector2f { x: self.x + other.x,
                   y: self.y + other.y }
    }
}

impl Sub for Vector2f {
    type Output = Vector2f;

    fn sub(self, other : Vector2f) -> Vector2f {
        Vector2f { x: self.x - other.x,
                   y: self.y - other.y }
    }
}

impl Div for Vector2f {
    type Output = Vector2f;

    fn div(self, other : Vector2f) -> Vector2f {
        Vector2f { x: self.x / other.x,
                   y: self.y / other.y }
    }
}

impl Div<f32> for Vector2f {
    type Output = Vector2f;

    fn div(self, divider : f32) -> Vector2f {
        let factor = 1.0 / divider;
        
        Vector2f { x: self.x * factor,
                   y: self.y * factor }
    }
}

impl Mul for Vector2f {
    type Output = Vector2f;

    fn mul(self, other : Vector2f) -> Vector2f {
        Vector2f { x: self.x * other.x,
                   y: self.y * other.y }
    }
}

impl Mul<f32> for Vector2f {
    type Output = Vector2f;

    fn mul(self, other : f32) -> Vector2f {
        Vector2f { x: self.x * other,
                   y: self.y * other }
    }
}

impl PartialEq for Vector2f {
    fn eq(&self, other : &Vector2f) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other : &Vector2f) -> bool {
        ! self.eq( other )
    }
}
