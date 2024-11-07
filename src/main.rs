mod map;
mod enemy;
mod inventory;

use crate::map::Map;
use crate::enemy::Enemy;
use crate::inventory::Inventory;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use std::io::{self, Write};
use std::time::Duration;

struct Player {
    health: i32,
    inventory: Inventory,
    position: (usize, usize),
}

impl Player {
    fn new() -> Self {
        Player {
            health: 100,
            inventory: Inventory::new(),
            position: (1, 1),
        }
    }

    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        println!("Você tomou {} de dano! Vida restante: {}", damage, self.health);
    }

    fn move_player(&mut self, dx: i32, dy: i32, map: &Map) {
        let new_x = (self.position.0 as i32 + dx) as usize;
        let new_y = (self.position.1 as i32 + dy) as usize;

        if map.is_walkable(new_x, new_y) {
            self.position = (new_x, new_y);
        } else {
            println!("Não é possível andar para lá!");
        }
    }
}

fn main() -> crossterm::Result<()> {
    let mut map = Map::generate(10, 10);
    let mut player = Player::new();
    let mut enemy = Enemy::new(10, 10);

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();

    loop {
        execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        map.display(&player.position, &enemy.position);
        println!("Use as teclas W, A, S, D para mover. Aperte Q para sair.");

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('w') => player.move_player(0, -1, &map),
                    KeyCode::Char('a') => player.move_player(-1, 0, &map),
                    KeyCode::Char('s') => player.move_player(0, 1, &map),
                    KeyCode::Char('d') => player.move_player(1, 0, &map),
                    KeyCode::Char('k') => enemy.take_damage(15),
                    KeyCode::Char('q') => break,
                    _ => (),
                }
            }
        }

        if enemy.position == player.position {
            player.take_damage(10);

            if player.health <= 0 {
                println!("Game Over!");
                break;
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
