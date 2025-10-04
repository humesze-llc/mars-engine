use crossbeam::channel::{unbounded, Sender, Receiver};
use mars_window::{WindowHandle, WindowEvent, UserMsg, WindowConfig};
use mars_foundation::error::Result;

#[derive(Clone)]
pub struct TestHarness {
    pub tx_events: Sender<WindowEvent>,
    pub rx_user: Receiver<UserMsg>,
}

pub fn start(_cfg: WindowConfig) -> (Result<WindowHandle>, TestHarness) {
    let (tx_events, rx_events) = unbounded::<WindowEvent>();
    let (tx_user, rx_user) = unbounded::<UserMsg>();

    let harness = TestHarness { tx_events, rx_user };
    let handle = WindowHandle { tx_user, rx_events };

    (Ok(handle), harness)
}