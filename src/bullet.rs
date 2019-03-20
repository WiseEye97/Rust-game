use piston_window::*;
use piston_window::types::Color;
use crate::my_draw::draw_rectangle;
use crate::opponent::*;
use crate::game::*;
use std::collections::{HashMap,LinkedList};

pub enum Collision{
    TargetCollision((u32,u32)),
    OutOfBoard(u32),
    NoCollsion
}

pub struct Bullet{
    id : u32,
    pub x : u32,
    pub y : u32,
    idling : f64,

}

const BULLET_COLOR : Color = [0.3,0.7,0.5,1.0];
const WAITIING_TIME : f64 = 0.05;


impl Bullet{
    pub fn new(id : u32,x : u32,y : u32) -> Bullet{
        Bullet {id,x,y,idling : 0.0}
    }
    pub fn draw_bullet(&self,con: &Context, g: &mut G2d){
        draw_rectangle( BULLET_COLOR, self.x as i32, self.y as i32, 10, 10, con, g);
    }

    pub fn move_bullet(&mut self,delta_time: f64,targets: & HashMap<u32,Opponent>) -> Collision {

        self.idling += delta_time;

        if self.idling > WAITIING_TIME {
            if self.y > 10 {
                self.y -= 10;
                for val in targets.values(){
                    if Game::do_overlap(&Object::Bullet(self), &Object::Target(val)){
                        return Collision::TargetCollision((self.id,val.id));
                    }    
                }    
            }
            else{
                return Collision::OutOfBoard(self.id);
            }
            self.idling = 0.0;
        }
        Collision::NoCollsion
    }
}