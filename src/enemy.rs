use rand::Rng;

pub struct Enemy {
    pub health: i32,
    pub position: (usize, usize),
}

impl Enemy {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(1..width); 
        let y = rng.gen_range(1..height); 
        
        Enemy {
            health: 50,
            position: (x, y),
        }
    }
    

    pub fn position(&self) -> (usize, usize) {
        self.position
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if self.health <= 0 {
            println!("O inimigo foi derrotado!");
        }
    }
}
