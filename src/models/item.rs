use graphics::{Context, rectangle, polygon, Transformed};
use opengl_graphics::GlGraphics;
use rand;
use rand::Rng;

use piston::window::Size;
use crate::color;
use crate::geom;
use super::GameObject;

const ITEM_RADIUS: f64 = 10.0;

pub struct Item {
    pub health: u32,
    pub pos: geom::Position,
    pub size: f64
}

impl Item {
    pub fn new(x: f64, y: f64) -> Item {
        Item {
            health: 1,
            pos: geom::Position::new(x, y),
            size: ITEM_RADIUS * 2.0,
        }
    }

    pub fn new_rand(max_x: f64, max_y: f64) -> Item {
        let mut rng = rand::thread_rng();
        let randx = rng.gen_range(0.0, max_x);
        let randy = rng.gen_range(0.0, max_y);
        Item::new(randx, randy)
    }
}


impl GameObject for Item {
    fn position(&self) -> &geom::Position { &self.pos }
    fn radius(&self) -> f64 { self.size / 2.0 }

    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render the enemy as a little square
        let radius = self.radius();
        let circle = rectangle::Rectangle::new_round_border(color::RED, radius, 1.0);
        let transform = ctxt.transform.trans(self.pos.x, self.pos.y)
            .trans(-radius, -radius);

        circle.draw([0.0, 0.0, radius * 2.0, radius * 2.0], &ctxt.draw_state, transform, gl);
    }

    // fn update(&mut self, dt: f64, size: Size) {
    //     // Only move every <MOVE_TTL> seconds.
    //     self.move_ttl -= dt;
    //     if self.move_ttl <= 0.0 {
    //         // Randomly move in a random direction.
    //         let radius = self.radius();
    //         let mut rng = rand::thread_rng();

    //         self.pos.x += rng.gen_range(0.0, MOVE_RADIUS * 2.0) - MOVE_RADIUS;
    //         self.pos.y += rng.gen_range(0.0, MOVE_RADIUS * 2.0) - MOVE_RADIUS;

    //         geom::restrict_to_bounds(
    //             &mut self.pos,
    //             [radius, radius, f64::from(size.width), f64::from(size.height)]
    //         );

    //         // Don't move outside the bounds of the window.
    //         self.move_ttl = MOVE_TTL;
    //     }
    // }
}