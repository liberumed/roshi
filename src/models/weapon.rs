use graphics::{Context, rectangle, ellipse, Transformed};
use opengl_graphics::GlGraphics;

use piston::window::Size;
use crate::color;
use crate::geom;
use super::GameObject;

pub struct Weapon {
    pub pos: geom::Position,
    pub dir: geom::Direction,
    pub size: f64,
    pub ttl: f64,
}

const WEAPON_SPEED: f64 = 3.0;
const WEAPON_SIZE: f64 = 10.0;
// Number of seconds til we can delete this bullet from the screen (if it hasn't
// collided with an enemy yet).
const WEAPON_LIFETIME: f64 = 0.3;

impl Weapon {
    pub fn new(x: f64, y: f64, dir: geom::Direction) -> Weapon {
        Weapon {
            dir,
            pos: geom::Position::new(x, y),
            size: WEAPON_SIZE,
            ttl: WEAPON_LIFETIME
        }
    }

    pub fn radius(&self) -> f64 {
        self.size / 2.0
    }
}

impl GameObject for Weapon {
    fn position(&self) -> &geom::Position { &self.pos }
    fn radius(&self) -> f64 { WEAPON_SIZE }

    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        let transform = ctxt.transform.trans(self.pos.x, self.pos.y);
        let radius = self.radius();
        let square = rectangle::square(0.0, 0.0, self.size);
        // ellipse(color::WHITE, [0.0, 0.0, radius, radius], transform, gl);

        rectangle(color::WHITE, square, transform, gl);
    }

    fn update(&mut self, dt: f64, _: Size) {
        self.ttl -= dt;
        // Move the bullet in the direction the player was facing.
        match self.dir {
            geom::Direction::EAST => self.pos.x += WEAPON_SPEED,
            geom::Direction::NORTH => self.pos.y -= WEAPON_SPEED,
            geom::Direction::WEST => self.pos.x -= WEAPON_SPEED,
            geom::Direction::SOUTH => self.pos.y += WEAPON_SPEED,
        }
    }
}