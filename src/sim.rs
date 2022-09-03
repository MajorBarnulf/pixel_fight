use std::{f32::consts::PI, time::Duration};

use rand::{prelude::IteratorRandom, random};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use speedy2d::{color::Color, dimen::Vec2};

#[derive(Debug)]
pub struct Team {
    initial_unit_count: usize,
    initial_position: Vec2,
    initial_radius: f32,
    color: Color,
}

impl Team {
    pub fn new(
        color: Color,
        initial_position: Vec2,
        initial_radius: f32,
        initial_unit_count: usize,
    ) -> Self {
        Self {
            color,
            initial_position,
            initial_radius,
            initial_unit_count,
        }
    }
}

pub enum UnitAction {
    Displace(usize, Vec2),
    SetTarget(usize, Option<usize>),
    Kill(usize, usize),
}

#[derive(Debug, Clone)]
pub struct Unit {
    id: usize,
    team_id: usize,
    position: Vec2,
    target_id: Option<usize>,
    alive: bool,
    speed: f32,
}

fn rand_speed() -> f32 {
    Unit::SPEED * (1. + (random::<f32>() * 2. - 1.) * Unit::SPEED_RANDOMNESS)
}

impl Unit {
    pub fn new(team_id: usize, id: usize, position: Vec2) -> Self {
        Self {
            id,
            alive: true,
            position,
            target_id: None,
            team_id,
            speed: rand_speed(),
        }
    }

    pub fn displace(&mut self, movement: Vec2) {
        self.position = self.position + movement;
    }

    pub fn set_target(&mut self, target: Option<usize>) {
        self.target_id = target;
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }

    pub fn tick(&self, units: &[Unit], delta: Duration) -> Option<UnitAction> {
        self.is_alive().then(|| {
            if let Some(target_id) = self.target_id {
                let other = &units[target_id];
                if self.is_in_range(other) {
                    UnitAction::Kill(self.id, other.id)
                } else {
                    UnitAction::Displace(
                        self.id,
                        self.direction_to(other) * self.speed * delta.as_secs_f32(),
                    )
                }
            } else {
                let target = units
                    .iter()
                    .filter(|u| (u.team_id != self.team_id) && u.is_alive())
                    .map(|u| u.id)
                    .collect::<Box<[usize]>>()
                    .iter()
                    .choose(&mut rand::thread_rng())
                    .cloned();
                UnitAction::SetTarget(self.id, target)
            }
        })
    }

    pub fn position(&self) -> &Vec2 {
        &self.position
    }

    pub fn color<'a>(&self, teams: &'a [Team]) -> &'a Color {
        &teams[self.team_id].color
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    const REACH: f32 = 10.;
    const SPEED: f32 = 20.;
    const SPEED_RANDOMNESS: f32 = 0.5;

    fn is_in_range(&self, other: &Self) -> bool {
        (self.position - other.position).magnitude_squared() < (Self::REACH.powf(2.))
    }

    fn direction_to(&self, other: &Self) -> Vec2 {
        (other.position - self.position)
            .normalize()
            .unwrap_or_else(|| (0., 0.).into())
    }
}

fn random_nearby_pos(center: Vec2, radius: f32) -> Vec2 {
    let Vec2 { x, y } = center;
    let angle = rand::random::<f32>() * 2. * PI;
    let module = rand::random::<f32>() * radius;
    Vec2 {
        x: x + angle.cos() * module,
        y: y + angle.sin() * module,
    }
}

#[derive(Debug)]
pub struct Simulation {
    teams: Vec<Team>,
    units: Vec<Unit>,
}

impl Simulation {
    pub fn new(desired_teams: impl IntoIterator<Item = Team>) -> Self {
        let mut teams = vec![];
        let mut units = vec![];

        for team in desired_teams.into_iter() {
            for _ in 0..team.initial_unit_count {
                let position = random_nearby_pos(team.initial_position, team.initial_radius);
                let unit = Unit::new(teams.len(), units.len(), position);
                units.push(unit);
            }
            teams.push(team);
        }

        Self { teams, units }
    }

    pub fn tick(&mut self, delta: Duration) {
        let actions = self
            .units
            .par_iter()
            .map(|unit| unit.tick(&self.units, delta))
            .collect::<Vec<_>>();

        for action in actions.into_iter().flatten() {
            {
                match action {
                    UnitAction::Displace(id, movement) => self.units[id].displace(movement),
                    UnitAction::SetTarget(id, target) => self.units[id].set_target(target),
                    UnitAction::Kill(id, target) => {
                        self.units[id].set_target(None);
                        self.units[target].kill()
                    }
                }
            }
        }
    }

    pub fn units(&self) -> &[Unit] {
        &self.units
    }

    pub fn teams(&self) -> &[Team] {
        &self.teams
    }
}
