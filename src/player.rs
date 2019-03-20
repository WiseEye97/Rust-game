use piston_window::*;
use piston_window::types::Color;

use crate::my_draw::draw_rectangle;

const PLAYERSIZE : (u32,u32) = (100,10);
const STEP : u32 = 10;



pub enum Direction{
    Right,
    Left
}

pub struct Player {
    pub x : u32,
    pub y : u32,
    lim : u32,
}

impl Player{
    pub fn new(x : u32,y : u32,lim : u32) -> Player {
        Player {x,y,lim}
    }
    fn move_left(& mut self){
        if self.x > 0 {
            self.x = self.x - (1*STEP);
        }      
    }
    fn move_right(& mut self){
        if self.x + PLAYERSIZE.0 < self.lim {
            self.x = self.x + (1*STEP);
        }
    }
    pub fn move_player(& mut self,dir : Direction){
        match dir{
            Direction::Right => self.move_right(),
            Direction::Left => self.move_left(),
        }
    }
    pub fn draw(& mut self,con: &Context, g: &mut G2d){
        draw_rectangle([0.7,0.4,0.4,0.9],self.x as i32 ,self.y as i32,PLAYERSIZE.0 as i32,PLAYERSIZE.1 as i32,con,g);
    }
}