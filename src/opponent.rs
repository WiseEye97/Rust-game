use piston_window::*;
use piston_window::types::Color;
use crate::my_draw::draw_rectangle;
use crate::game::Object;


const OPPONENT_COLOR : Color = [0.2,0.9,0.9,0.3];
const WAITIING_TIME : f64 = 0.1;

#[derive(Debug)]
pub enum EnemyType{
    Circle,
    Rect,
    Triangle,
}
#[derive(Debug)]
pub struct Opponent{
    pub id : u32,
    etype: EnemyType,
    x : u32,
    y : u32,
    idling : f64,
}

impl Opponent{
    pub fn new(t: EnemyType,x : u32,y: u32,id : u32) -> Opponent{
        Opponent {etype : t,x,y,id,idling : 0.0}
    }
    pub fn draw_opponent(&self,con: &Context, g: &mut G2d){
        match self.etype{
            EnemyType::Rect => draw_rectangle(OPPONENT_COLOR, self.x as i32, self.y as i32, 40 as i32, 20, con, g),
            _ => (),
        }
    }
    pub fn get_width_height(&self) -> (u32,u32) {
        match self.etype{
            EnemyType::Rect => (40,20),
            _ => (10,10),
        }
    }
    pub fn do_overlap(op1: &Opponent,op2: &Opponent) -> bool {
        let (size1, size2) = (op1.get_width_height() , op2.get_width_height());

        if (op1.x > (op2.x + size2.0)) || (op2.x > (op1.x + size1.0)) {
            return false;
        }

        if (op1.y > op2.y + size2.1) || (op2.y > op1.y + size1.1) {
            return false;
        }

        true
    }

    pub fn change_x(&mut self,x : u32){
        self.x = x;
    }
    pub fn change_y(&mut self,y : u32){
        self.y = y;
    }

    pub fn move_opponent(&mut self,delta_time: f64) -> bool{
        self.idling += delta_time;


        if self.idling > WAITIING_TIME {
            self.y += 5;

            if self.y > 380{
                
                return true;
            }

            self.idling = 0.0;
        }
  
        false
    }

    pub fn get_id(& self) -> u32{
        self.id
    }

    pub fn get_x(& self) -> u32{
        self.x
    }

    pub fn get_y(& self) -> u32{
        self.y
    }
}