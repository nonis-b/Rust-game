pub struct FrameStats {
    num: u32,
    time_ms: u32,
    time_iter_ms: u32,
}

const UPDATE_MS: u32 = 2000;

impl FrameStats {

    pub fn init() -> FrameStats {
        return FrameStats {num: 0, time_ms: 0, time_iter_ms: 0};
    }

    pub fn tick_and_print(&mut self, time_ms: u32) {
        self.num += 1;
        self.time_iter_ms += (time_ms as i64 - self.time_ms as i64) as u32;
        self.time_ms = time_ms;
        if self.time_iter_ms > UPDATE_MS {
            let fps = self.num / (UPDATE_MS / 1000);
            println!("FPS: {}", fps);
            self.num = 0;
            self.time_iter_ms = 0;
        }
    }
}