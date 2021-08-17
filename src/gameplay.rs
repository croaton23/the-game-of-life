
use rand::Rng;

pub struct Field {
    pub size: usize,
    pub field: Vec<Vec<i32>>,
}

impl Field {
    pub fn new(size: usize) -> Field {
        Field { 
            size,
            field: vec![vec![0; size]; size]
        }
    }
}

pub trait Game {
    fn init(&mut self);
    fn tick(&mut self, dt:f64);
}

impl Game for Field {
    fn init(&mut self) {
        let x = rand::thread_rng().gen_range(0..self.size);
        let y = rand::thread_rng().gen_range(0..self.size);
        self.field[x][y] = 1;
    }

    fn tick(&mut self, dt:f64) {
        
    }
}