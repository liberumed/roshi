// kokoro = "heart; mind; mentality; emotions; feelings"
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::{GlyphCache, TextureSettings};
use piston::input::*;

mod color;
mod geom;
mod gfx;
mod models;
pub mod config;

use crate::gfx::utils::{draw_center, draw_text};
use crate::models::{GameObject};
use crate::models::weapon::Weapon;
use crate::models::enemy::Enemy;
use crate::models::player::Player;
use crate::models::item::Item;

const FIRE_COOLDOWN: f64 = 0.1; // Only allow user to shoot 10 bullets/sec.

enum GameStatus {
    // Normal fighting mode
    Normal,
    // Player died
    Died,
    // Player won!
    Win
}

struct GameState {
    debug_mode: bool,
    // Overall game state
    game_status: GameStatus,
    // User shooting state
    fire_weapon: bool,
    fire_cooldown: f64,
}

struct Cell {
    x: u32,
    y: u32
}

pub struct App<'a> {
    pub window: config::GraphicsConfig,
    glyph_cache: GlyphCache<'a>,
    player: Player,
    enemies: Vec<Enemy>,
    items: Vec<Item>,
    weapons: Vec<Weapon>,
    score: u32,
    size_score: f64,
    // Game-wide events
    state: GameState,
}

impl<'a> App<'a> {
    pub fn new(window: config::GraphicsConfig) -> App<'a> {
        let size = window.size;

        let (x, y) = (f64::from(size.width / 2.0),
                      f64::from(size.height / 2.0));

        let player = Player::new(x, y);

        let state = GameState {
            debug_mode: false,
            fire_weapon: false,
            fire_cooldown: 0.0,
            game_status: GameStatus::Normal
        };

        // Load font(s) used in the game.
        let glyph_cache = GlyphCache::new("./assets/fonts/PxPlus_IBM_VGA8.ttf", (), TextureSettings::new())
            .expect("Unable to load font");

        App {
            glyph_cache,
            player,
            state,
            window,
            weapons: Vec::new(),
            enemies: Vec::new(),
            items: Vec::new(),
            score: 0,
            size_score: 0.0,
        }
    }

    fn reset(&mut self) {
        self.state.game_status = GameStatus::Normal;
        self.score = 0;
        self.size_score = self.player.size;
        self.enemies.clear();
        self.items.clear();
    }

    pub fn input(&mut self, button: &Button, is_press: bool) {
        if is_press {
            if let Button::Keyboard(key) = *button {
                match key {
                    Key::W => self.player.start_move(geom::Direction::NORTH),
                    Key::S => self.player.start_move(geom::Direction::SOUTH),
                    Key::A => self.player.start_move(geom::Direction::WEST),
                    Key::D => self.player.start_move(geom::Direction::EAST),
                    Key::Space => {
                        if self.state.fire_cooldown <= 0.0 {
                            self.state.fire_cooldown = FIRE_COOLDOWN;
                            self.state.fire_weapon = true;
                        }
                    },
                    // Toggle debug mode.
                    // Key::D => {
                    //     self.state.debug_mode = !self.state.debug_mode;
                    //     println!("Debug mode: {}", self.state.debug_mode);
                    // },
                    // Reset game
                    Key::Return => {
                        match self.state.game_status {
                            GameStatus::Died => self.reset(),
                            GameStatus::Win => self.reset(),
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
        } else if let Button::Keyboard(key) = *button {
            match key {
                Key::W => self.player.stop_move(geom::Direction::NORTH),
                Key::S => self.player.stop_move(geom::Direction::SOUTH),
                Key::A => self.player.stop_move(geom::Direction::WEST),
                Key::D => self.player.stop_move(geom::Direction::EAST),
                _ => (),
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let weapons = &self.weapons;
        let enemies = &self.enemies;
        let items = &self.items;
        let player = &self.player;
        let gc = &mut self.glyph_cache;
        let state = &self.state;

        let debug_mode = self.state.debug_mode;
        let score = self.score;
        let size_score = self.size_score;
        let size = self.window.size;

        // let square = rectangle::square(0.0, 0.0, 24.0);
        // let rotation = self.rotation;
        // let (x, y) = (
        //     args.window_size[0] / 2.0,
        //     args.window_size[1] / 2.0
        // );

        self.window.gl.draw(args.viewport(), |c, gl| {
            use graphics::*;

            // Clear the screen.
            clear(crate::color::BLACK, gl);

            // Check game status
            match state.game_status {
                GameStatus::Died => {
                    draw_center("YOU DIED!", 32, [f64::from(size.width), f64::from(size.height)], gc, &c, gl);
                    return;
                },
                GameStatus::Win => {
                    draw_center("YOU WIN!", 32, [f64::from(size.width), f64::from(size.height)], gc, &c, gl);
                    return;
                },
                _ => (),
            }

            // Render the current score
            let score_str = format!("Score: {}", score);
            draw_text(score_str.as_str(), [10.0, 26.0], 16, gc, &c, gl);

            let cell = Cell {
                x: 1,
                y: 1
            };
            let cell_str = format!("Cell: {x}:{y}", x = cell.x, y = cell.y);
            draw_text(cell_str.as_str(), [10.0, 46.0], 16, gc, &c, gl);

            for weapon in weapons.iter() {
                weapon.render(&c, gl);
            }


            for enemy in enemies.iter() {
                enemy.render(&c, gl);
            }

            for item in items.iter() {
                item.render(&c, gl);
            }

            player.render(&c, gl);

            if debug_mode {
                player.render_dbg(&c, gl);
            }

            // let transform = c.transform.trans(x, y)
            //                             .rot_rad(rotation)
            //                             .trans(-12.0, -12.0);
            // rectangle(crate::color::YELLOW, square, transform, gl)
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self.state.game_status {
            GameStatus::Died => return,
            GameStatus::Win => return,
            _ => (),
        }

        let size = self.window.size;

        // Handle game events
        if self.state.fire_cooldown > 0.0 {
            self.state.fire_cooldown -= args.dt;
        }

        if self.state.fire_weapon {
            self.state.fire_weapon = false;
            self.weapons.push(
                Weapon::new(self.player.pos.x, self.player.pos.y, self.player.dir)
            );

            self.weapons.push(
                Weapon::new(self.player.pos.x - 15.0, self.player.pos.y - 15.0, self.player.dir)
            );

            // self.weapons.push(
            //     match self.player.dir {
            //         geom::Direction::WEST =>  Weapon::new(self.player.pos.x - 15.0, self.player.pos.y - 15.0, self.player.dir),
            //         geom::Direction::NORTH => Weapon::new(self.player.pos.x, self.player.pos.y - 15.0, self.player.dir),
            //         geom::Direction::EAST =>  Weapon::new(self.player.pos.x - 15.0, self.player.pos.y, self.player.dir),
            //         geom::Direction::SOUTH => Weapon::new(self.player.pos.x, self.player.pos.y - 15.0, self.player.dir),
            //     }
            // );
        }

        for weapon in &mut self.weapons {
            weapon.update(args.dt, size);
            // Did bullet collide with any enemies
            for enemy in &mut self.enemies {
                if weapon.collides(enemy) {
                    // Destroy bullet
                    weapon.ttl = 0.0;
                    // Destroy enemy
                    enemy.health -= 1;
                    self.score += 5;
                }
            }
        }
        // Remove bullets that have outlived their TTL
        self.weapons.retain(|weapon| weapon.ttl > 0.0);

        self.enemies.retain(|enemy| enemy.health > 0);
        self.items.retain(|item| item.health > 0);

        // Update player & enemies
        self.player.update(args.dt, size);

         // If number of enemies is zero... spawn more!
        if self.enemies.is_empty() {
            let size = self.window.size;
            for _ in 0..10 {
                self.enemies.push(Enemy::new_rand(f64::from(size.width), f64::from(size.height)));
            }
        }

        if self.items.is_empty() {
            let size = self.window.size;
            for _ in 0..10 {
                self.items.push(Item::new_rand(f64::from(size.width), f64::from(size.height)));
            }
        }

        for enemy in &mut self.enemies {
            enemy.update(args.dt, size);
            // If the player collides with an enemy, game over!
            if enemy.collides(&self.player) {
                self.state.game_status = GameStatus::Died;
            }
        }

        if self.player.size < self.size_score {
            self.player.size = self.size_score;
        }

        for item in &mut self.items {
            // enemy.update(args.dt, size);
            // If the player collides with an enemy, game over!
            if item.collides(&self.player) {
                item.health -= 1;
                // self.score += 5;
                self.size_score += 5.0;
            }
        }

        // Did we kill all the enemies?
        if self.score >= 100 {
            self.state.game_status = GameStatus::Win;
        }
    }
}