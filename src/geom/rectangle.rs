use std::cmp::{max, min, PartialEq};

use super::Point;
use super::Size;

pub struct Rectangle {
    pub x : i32,
    pub y : i32,
    pub width : i32,
    pub height : i32
}

impl Rectangle {
    pub fn new(x : i32, y : i32, width : i32, height : i32) -> Rectangle {
        Rectangle { x: x,
                    y: y,
                    width: width,
                    height: height }
    }

    pub fn empty() -> Rectangle {
        Rectangle { x: 0,
                    y: 0,
                    width: 0,
                    height: 0 }
    }

    pub fn union(left : &Rectangle, right : &Rectangle) -> Rectangle {
        let x = min( left.x, right.x );
        let y = min( left.y, right.y );
        
        Rectangle {
            x      : x,
            y      : y,
            width  : max( left.get_right() , right.get_right() ) - x,
            height : max( left.get_bottom(), right.get_bottom() ) - y
        }
    }

    pub fn get_top(&self) -> i32 {
        self.y
    }

    pub fn get_right(&self) -> i32 {
        self.x + self.width
    }

    pub fn get_bottom(&self) -> i32 {
        self.y + self.height
    }

    pub fn get_left(&self) -> i32 {
        self.x
    }

    pub fn get_location(&self) -> Point {
        Point { x: self.x,
                y: self.y }
    }

    pub fn get_center(&self) -> Point {
        Point { x: self.x + ( self.width / 2 ),
                y: self.y + ( self.height / 2 ) }
    }

    pub fn size(&self) -> Size {
        Size { width: self.width,
               height: self.height }
    }

    pub fn is_empth(&self) -> bool {
        self.width == 0
            &&
            self.height == 0
            &&
            self.x == 0
            &&
            self.y == 0
    }

    pub fn contains(&self, x : i32, y : i32) -> bool {
        self.x <= x
            &&
            x < ( self.x + self.width )
            &&
            self.y <= y
            &&
            y < ( self.y + self.height )
    }

    pub fn intersects(&self, other : &Rectangle) -> bool {
        other.get_left() < self.get_right()
            &&
            self.get_left() < other.get_right()
            &&
            other.get_top() < self.get_bottom()
            &&
            self.get_top() < other.get_bottom()
    }

    pub fn inflate(&mut self, horizontal_amount : i32, vertical_amount : i32) {
        self.x -= horizontal_amount;
        self.y -= vertical_amount;

        self.width += horizontal_amount;
        self.height += vertical_amount;
    }

    fn debug_display_string(&self) -> String {
        format!( "{} {} {} {}", self.x, self.y, self.width, self.height )
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other : &Rectangle) -> bool {
        self.x == other.x
            &&
            self.y == other.y
            &&
            self.width == other.width
            &&
            self.height == other.height
    }

    fn ne(&self, other : &Rectangle) -> bool {
        ! self.eq( other )
    }
}
