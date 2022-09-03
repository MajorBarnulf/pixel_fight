use speedy2d::dimen::Vec2;

#[derive(Debug)]
pub struct View {
    origin: Vec2,
}

impl View {
    pub fn origin(&self) -> &Vec2 {
        &self.origin
    }

    pub fn displace(&mut self, movement: Vec2) {
        self.origin = self.origin + movement;
    }
}

impl Default for View {
    fn default() -> Self {
        let origin = (0., 0.).into();
        Self { origin }
    }
}
