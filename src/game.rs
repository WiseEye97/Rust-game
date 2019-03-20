use crate::player::*;
use crate::opponent::*;
use crate::bullet::*;
use piston_window::*;
use piston_window::types::Color;
use std::collections::{HashMap,LinkedList};

use rand::{thread_rng, Rng};

const MOVING_PERIOD: f64 = 0.1;

pub enum Object<'a> {
    Target(&'a Opponent),
    Bullet(&'a Bullet),
}

impl<'a> Object<'a>{
    pub fn get_width_height(&self) -> (u32,u32){
        match self{
            Object::Bullet(b) => (10,10),
            Object::Target(op) => op.get_width_height(),
        }
    }
    pub fn get_x(&self) -> u32{
        match self{
            Object::Bullet(b) => b.x,
            Object::Target(op) => op.get_x(),
        }
    }
    pub fn get_y(&self) -> u32{
        match self{
            Object::Bullet(b) => b.y,
            Object::Target(op) => op.get_y(),
        }
    }
}

enum KeyType {
    Movement(Direction),
    Shot,
}

pub struct Game{
    width : u32,
    height : u32,
    player : Player,
    pub waiting_time: f64,
    nid1 : u32,
    nid2 : u32,
    count : u32,
    opponents : HashMap<u32,Opponent>,
    bullets : HashMap<u32,Bullet>,
    target_positions : HashMap<(u32,u32),u32>,
}

impl Game{
    pub fn new(width : u32,height : u32) -> Game {
        Game {count : 0,nid1 : 0,nid2 : 0,target_positions: HashMap::new(),waiting_time: 0.0,width,height,player : Player::new(width/2,height - 50,width),opponents : HashMap::new(),bullets : HashMap::new()}
    }
    pub fn key_pressed(&mut self, key: Key){
        let dir = match key{
            Key::Left => Some(KeyType::Movement(Direction::Left)),
            Key::Right => Some(KeyType::Movement(Direction::Right)),
            Key::Space => Some(KeyType::Shot),
            _ => None,
        };

        match dir{
            Some(KeyType::Movement(d)) => self.player.move_player(d),
            Some(KeyType::Shot) => {
                let id = self.nid2;
                self.nid2 += 1;
                self.bullets.insert(id, Bullet::new(id, self.player.x + 48, self.player.y - 10));
            },
            _ => ()
        }
    }

    pub fn do_overlap(op1: &Object,op2: &Object) -> bool {
        let (size1, size2) = (op1.get_width_height() , op2.get_width_height());
        let (l1,l2) = ((op1.get_x(),op1.get_y()), (op2.get_x(),op2.get_y()));
        let (p1,p2) = ((l1.0 + size1.0,l1.1 + size1.1),(l2.0 + size2.0,l2.1 + size2.1));
        if (l1.0 > p2.0) || (l2.0> (p1.0)) {
            return false;
        }

        if (l1.1 > p2.1) || (l2.1 > p1.1) {
            return false;
        }

        true
    }
    pub fn draw_board(&mut self,con: &Context, g: &mut G2d){
        self.player.draw(con,g);
        for (_key, val) in self.opponents.iter() {
            //print!("Opponent -> {:?}",val);
            val.draw_opponent(con,g);
        }
        for (_key, val) in self.bullets.iter() {
            //print!("Opponent -> {:?}",val);
            val.draw_bullet(con,g);
        }
    }

    fn can_place(&mut self,o : &Opponent) -> bool {
        for (_key, val) in self.opponents.iter() {
            if Opponent::do_overlap(val, o) {
                return false;
            }
        }

        true
    }

    pub fn spawn_opponent(&mut self){
        let mut rng = thread_rng();
        let id = self.nid1;

        self.nid1 += 1;

        let new_x = rng.gen_range(1, self.width - 41);
        let new_y = rng.gen_range(1, self.height - 100);

        let mut new_opponent = Opponent::new(EnemyType::Rect, new_x , new_y,id);

        while !self.can_place(&new_opponent) {
            new_opponent.change_x(rng.gen_range(1, self.width - 41));
            new_opponent.change_y(rng.gen_range(1, self.height - 100));
        }

        self.target_positions.insert((new_opponent.get_x(),new_opponent.get_y()), id);
        self.opponents.insert(id,new_opponent);
    }

    fn move_opponents(&mut self,delta_time: f64){
        let mut to_delete : LinkedList<u32> = LinkedList::new();    

        for val in self.opponents.values_mut(){
            if val.move_opponent(delta_time) {
                to_delete.push_back(val.get_id());
            }
        }

        for k in to_delete.iter(){
            self.opponents.remove(k);
        }

    }
    fn move_bullets(&mut self,delta_time: f64){
        let mut to_delete = LinkedList::new();
        let mut to_delete2 = LinkedList::new();

        for val in self.bullets.values_mut(){
            match val.move_bullet(delta_time,&self.opponents){
                Collision::TargetCollision((build,targetid)) => {
                    to_delete.push_back((build,targetid));
                    self.count += 1;
                }
                Collision::OutOfBoard(id) => to_delete2.push_back(id),
                Collision::NoCollsion => (), 
            }
        }

        for (build,targetid) in to_delete.iter() {
            self.bullets.remove(build);
            self.opponents.remove(targetid);
        }

        for id in to_delete2.iter() {
            self.bullets.remove(id);
        }
    }
    pub fn update(&mut self, delta_time: f64){
        self.move_opponents(delta_time);
        self.move_bullets(delta_time);    
    }

    pub fn get_count(&self) -> u32{
        self.count
    }

}