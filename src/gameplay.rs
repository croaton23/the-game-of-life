
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
        let mut copy:Vec<Vec<i32>> = vec![];
        for row in &self.field {
            copy.push(row.clone());
        }

        for i in 0..self.size {
            for j in 0..self.size {
                let mut neighbours:Vec<(i32,i32)> = vec![
                    (i as i32 - 1, j as i32  - 1), (i as i32 , j as i32  - 1), (i as i32  - 1, j as i32 ),
                    (i  as i32 + 1, j  as i32 - 1), (i  as i32 - 1, j  as i32 + 1),
                    (i  as i32 + 1, j  as i32 + 1), (i as i32 , j as i32  + 1), (i as i32  + 1, j as i32 )
                ];
                
                neighbours.retain(|&n| n.0 >= 0 && n.1 >= 0 && n.0 < self.size as i32 && n.1 < self.size as i32);
                let neighbour_values: Vec<i32> = neighbours.iter()
                    .map(|&n| copy[n.0 as usize][n.1 as usize])
                    .collect();

                let amount:i32 = neighbour_values.iter().sum();
                let alive:bool = copy[i][j] == 1;
                if (amount < 2 || amount > 3) && alive {
                    self.field[i][j] = 0;
                }
                if amount == 1 && !alive {
                    self.field[i][j] = 1;
                }
            }
        }
    }
}