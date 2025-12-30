//! Scroll state management

#[derive(Default, Clone)]
pub struct ScrollState {
    pub v_scroll: usize,
    pub h_scroll: usize,
    pub pending_g: bool,
}

impl ScrollState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.v_scroll = 0;
        self.h_scroll = 0;
        self.pending_g = false;
    }

    pub fn scroll_up(&mut self, amount: usize) {
        self.v_scroll = self.v_scroll.saturating_sub(amount);
    }

    pub fn scroll_down(&mut self, amount: usize, max: usize) {
        self.v_scroll = (self.v_scroll + amount).min(max);
    }

    pub fn scroll_left(&mut self, amount: usize) {
        self.h_scroll = self.h_scroll.saturating_sub(amount);
    }

    pub fn scroll_right(&mut self, amount: usize, max: usize) {
        self.h_scroll = (self.h_scroll + amount).min(max);
    }

    pub fn home(&mut self) {
        self.v_scroll = 0;
    }

    pub fn end(&mut self, max: usize) {
        self.v_scroll = max;
    }

    pub fn h_home(&mut self) {
        self.h_scroll = 0;
    }

    pub fn h_end(&mut self, max: usize) {
        self.h_scroll = max;
    }
}
