use std::time::Instant;

use speedy2d::{color::Color, dimen::Vec2, shape::Rectangle, window::WindowHandler};

use crate::{Simulation, View};

#[derive(Debug)]
pub struct App {
    sim: Simulation,
    view: View,
    last_tick: Instant,
}

impl App {
    pub fn new(sim: Simulation) -> Self {
        Self {
            sim,
            view: View::default(),
            last_tick: Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_tick);
        self.sim.tick(delta);
        self.last_tick = now;
    }
}

const UNIT_WIDTH: f32 = 4.;
const UNIT_SIZE: Vec2 = Vec2 {
    x: UNIT_WIDTH,
    y: UNIT_WIDTH,
};

impl WindowHandler for App {
    fn on_draw(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        graphics: &mut speedy2d::Graphics2D,
    ) {
        self.tick();
        graphics.clear_screen(Color::from_hex_rgb(0x202020));

        for unit in self.sim.units() {
            if !unit.is_alive() {
                continue;
            }
            let screen_position = unit.position() - self.view.origin();
            let tl = screen_position - (UNIT_SIZE / 2.);
            let br = screen_position + (UNIT_SIZE / 2.);
            graphics.draw_rectangle(Rectangle::new(tl, br), *unit.color(self.sim.teams()))
        }

        helper.request_redraw();
    }

    fn on_keyboard_char(
        &mut self,
        _h: &mut speedy2d::window::WindowHelper<()>,
        unicode_codepoint: char,
    ) {
        const MOVEMENT_STEP: f32 = 100.;

        match unicode_codepoint {
            'z' => self.view.displace((0., -MOVEMENT_STEP).into()),
            'q' => self.view.displace((-MOVEMENT_STEP, 0.).into()),
            's' => self.view.displace((0., MOVEMENT_STEP).into()),
            'd' => self.view.displace((MOVEMENT_STEP, 0.).into()),
            _ => (),
        }
    }
}
