use rand::Rng;

pub struct Map {
    grid: Vec<Vec<char>>,
}

impl Map {
    pub fn generate(width: usize, height: usize) -> Self {
        let mut grid = vec![vec!['.'; width]; height];
        let mut rng = rand::thread_rng();

        for _ in 0..(width * height / 5) {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            grid[y][x] = '#';
        }

        Map { grid }
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.grid.get(y).and_then(|row| row.get(x)) == Some(&'.')
    }

    pub fn display(&self, player_position: &(usize, usize), enemy_position: &(usize, usize)) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if (x, y) == *player_position {
                    print!("P ");
                } else if (x, y) == *enemy_position {
                    print!("E ");
                } else {
                    print!("{} ", cell);
                }
            }
            println!();
        }
    }
}
