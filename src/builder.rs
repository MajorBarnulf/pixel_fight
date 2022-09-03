use serde::{Deserialize, Serialize};
use speedy2d::color::Color;

use crate::Simulation;

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    color: (u8, u8, u8),
    position: (f32, f32),
    radius: f32,
    count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Descriptor {
    teams: Vec<Team>,
}

pub fn example_descriptor() -> Descriptor {
    Descriptor {
        teams: vec![
            Team {
                color: (255, 0, 0),
                count: 1000,
                position: (0., 0.),
                radius: 100.,
            },
            Team {
                color: (0, 255, 0),
                count: 1000,
                position: (0., 1000.),
                radius: 100.,
            },
            Team {
                color: (0, 0, 255),
                count: 1000,
                position: (800., 500.),
                radius: 100.,
            },
        ],
    }
}

impl Descriptor {
    pub fn unwrap(self) -> Vec<crate::Team> {
        let Self { teams } = self;
        teams
            .into_iter()
            .map(
                |Team {
                     color: (r, g, b),
                     position,
                     count,
                     radius,
                 }| {
                    crate::Team::new(Color::from_int_rgb(r, g, b), position.into(), radius, count)
                },
            )
            .collect()
    }
}

pub fn build(descr: Descriptor) -> Simulation {
    let teams = descr.unwrap();
    Simulation::new(teams)
}
