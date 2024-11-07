use rand::Rng;

pub struct Map {
    grid: Vec<Vec<char>>,  // Matriz que representa o mapa
}

impl Map {
    // Função para gerar o mapa
    pub fn generate(width: usize, height: usize) -> Self {
        let mut grid = vec![vec!['.'; width]; height];  // Inicializa o mapa com espaços vazios
        let mut rng = rand::thread_rng();

        // Coloca obstáculos aleatórios no mapa
        for _ in 0..(width * height / 5) {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            grid[y][x] = '#';  // Define um obstáculo
        }

        Map { grid }  // Retorna o mapa gerado
    }

    // Verifica se a posição é caminhável
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.grid.get(y).and_then(|row| row.get(x)) == Some(&'.')
    }

    // Exibe o mapa com o jogador e inimigo
    pub fn display(&self, player_position: &(usize, usize), enemy_position: &(usize, usize)) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if (x, y) == *player_position {
                    print!("P ");  // Exibe o jogador
                } else if (x, y) == *enemy_position {
                    print!("E ");  // Exibe o inimigo
                } else {
                    print!("{} ", cell);  // Exibe o terreno
                }
            }
            println!();
        }
    }
}
