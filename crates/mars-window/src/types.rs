use crossbeam::channel::{Sender, Receiver};

#[derive(Clone, Debug)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}
impl Default for WindowConfig {
    fn default() -> Self {
        Self { title: "Mars".into(), width: 1280, height: 720 }
    }
}

#[derive(Clone, Debug)]
pub enum WindowEvent {
    CloseRequested,
    Resized { width: u32, height: u32 },
    ScaleFactorChanged { scale: f64 },
    FocusChanged { focused: bool },
    Keyboard,
    Mouse,
    Redraw,
}

pub struct WindowEvents {
    pub queue: Vec<WindowEvent>,
}
impl WindowEvents {
    pub fn new() -> Self { Self { queue: Vec::new() } }
    pub fn drain(&mut self) -> impl Iterator<Item = WindowEvent> + '_ { self.queue.drain(..) }
    pub fn push(&mut self, ev: WindowEvent) { self.queue.push(ev); }
}

#[derive(Clone)]
pub struct WindowHandle {
    pub tx_user: Sender<UserMsg>,
    pub rx_events: Receiver<WindowEvent>,
}

#[derive(Clone, Debug)]
pub enum UserMsg { RequestRedraw }