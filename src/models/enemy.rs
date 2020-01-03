use graphics::{Context, rectangle, polygon, Transformed};
use opengl_graphics::GlGraphics;
use rand;
use rand::Rng;

use piston::window::Size;
use crate::color;
use crate::geom;
use super::GameObject;

// The max movement of the enemy in a rando direction.
const MOVE_RADIUS: f64 = 5.0;
const MOVE_TTL: f64 = 0.1; // 100 millisecond
const ENEMY_RADIUS: f64 = 10.0;

pub struct Enemy {
    pub health: u32,
    pub pos: geom::Position,
    pub size: f64,
    move_ttl: f64,
}

impl Enemy {
    pub fn new(x: f64, y: f64) -> Enemy {
        Enemy {
            health: 1,
            move_ttl: MOVE_TTL,
            pos: geom::Position::new(x, y),
            size: ENEMY_RADIUS * 2.0,
        }
    }

    pub fn new_rand(max_x: f64, max_y: f64) -> Enemy {
        let mut rng = rand::thread_rng();
        let randx = rng.gen_range(0.0, max_x);
        let randy = rng.gen_range(0.0, max_y);
        Enemy::new(randx, randy)
    }
}

impl GameObject for Enemy {
    fn position(&self) -> &geom::Position { &self.pos }
    fn radius(&self) -> f64 { self.size / 2.0 }

    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render the enemy as a little square
        let eye_rad = self.size/8.0;
        let eye = rectangle::Rectangle::new_round_border(color::RED, eye_rad, 1.0);

        let mouth = polygon::Polygon::new(color::RED);
        let points = [
            [0.0, 0.0],
            [eye_rad * 4.0, 0.0],
            [eye_rad * 3.5, eye_rad * 2.0],
            [eye_rad * 3.0, 0.0]
        ];

        let square = rectangle::square(0.0, 0.0, self.size);
        let radius = self.radius();
        let transform = ctxt.transform.trans(self.pos.x, self.pos.y)
            .trans(-radius, -radius);

        let eye_transform_1 = ctxt.transform.trans(self.pos.x + eye_rad, self.pos.y + eye_rad)
            .trans(-radius, -radius);

        let eye_transform_2 = ctxt.transform.trans(self.pos.x + 5.0 * eye_rad, self.pos.y + eye_rad)
            .trans(-radius, -radius);

        let mouth_transform = ctxt.transform
            .trans(self.pos.x + 2.0 * eye_rad, self.pos.y + 5.0 * eye_rad)
            .trans(-radius, -radius);

        rectangle(color::GREEN, square, transform, gl);

        eye.draw([0.0, 0.0, eye_rad * 2.0, eye_rad * 2.0], &ctxt.draw_state, eye_transform_1, gl);
        eye.draw([0.0, 0.0, eye_rad * 2.0, eye_rad * 2.0], &ctxt.draw_state, eye_transform_2, gl);

        mouth.draw(
            &points,
            &ctxt.draw_state,
            mouth_transform,
            gl
        );
    }

    fn render_dbg(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render collison box
        let radius = self.radius();
        let diam = radius * 2.0;

        let circle = rectangle::Rectangle::new_round_border(color::WHITE, radius, 1.0);
        // Center on x/y
        let transform = ctxt.transform
            .trans(self.pos.x, self.pos.y)
            .trans(-radius, -radius);

        circle.draw([0.0, 0.0, diam, diam], &ctxt.draw_state, transform, gl);
    }

    fn update(&mut self, dt: f64, size: Size) {
        // Only move every <MOVE_TTL> seconds.
        self.move_ttl -= dt;
        if self.move_ttl <= 0.0 {
            // Randomly move in a random direction.
            let radius = self.radius();
            let mut rng = rand::thread_rng();

            self.pos.x += rng.gen_range(0.0, MOVE_RADIUS * 2.0) - MOVE_RADIUS;
            self.pos.y += rng.gen_range(0.0, MOVE_RADIUS * 2.0) - MOVE_RADIUS;

            geom::restrict_to_bounds(
                &mut self.pos,
                [radius, radius, f64::from(size.width), f64::from(size.height)]
            );

            // Don't move outside the bounds of the window.
            self.move_ttl = MOVE_TTL;
        }
    }
}