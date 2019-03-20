use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    
    rectangle(
        color,
        [
            x as f64,
            y as f64,
            width as f64,
            height as f64,
        ],
        con.transform,
        g,
    );
}