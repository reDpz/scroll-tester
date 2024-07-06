pub struct Timer {
    pub time: f32,
    pub tracked_time: f32,
}

impl Timer {
    pub fn new(time: f32) -> Self {
        Timer {
            time,
            tracked_time: 0.0,
        }
    }

    pub fn tick(&mut self, delta: f32) {
        self.tracked_time += delta;
    }

    pub fn timeout(&self) -> bool {
        self.tracked_time >= self.time
    }

    pub fn tick_timeout(&mut self, delta: f32) -> bool {
        self.tick(delta);
        self.timeout()
    }

    pub fn reset(&mut self) {
        self.tracked_time = 0.0;
    }
    pub fn soft_reset(&mut self) {
        self.tracked_time -= self.time;
    }
}
