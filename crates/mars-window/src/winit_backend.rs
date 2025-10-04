use crossbeam::channel::{unbounded, Sender, Receiver};
use std::thread;
use mars_foundation::error::Result;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent as WinitWindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};
use crate::types::{UserMsg, WindowEvent, WindowHandle, WindowConfig};

struct WindowApp {
    cfg: WindowConfig,
    tx_events: Sender<WindowEvent>,
    _rx_user: Receiver<UserMsg>,
    window: Option<Window>,
}

impl ApplicationHandler<UserMsg> for WindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = WindowAttributes::default()
            .with_title(self.cfg.title.clone())
            .with_inner_size(PhysicalSize::new(self.cfg.width, self.cfg.height));
        if self.window.is_none() {
            if let Ok(w) = event_loop.create_window(attrs) {
                self.window = Some(w);
            }
        }
    }

    fn user_event(&mut self, _el: &ActiveEventLoop, event: UserMsg) {
        if let (UserMsg::RequestRedraw, Some(window)) = (event, &self.window) {
            window.request_redraw();
        }
    }

    fn window_event(&mut self, _el: &ActiveEventLoop, _id: winit::window::WindowId, event: WinitWindowEvent) {
        match event {
            WinitWindowEvent::CloseRequested => { let _ = self.tx_events.send(WindowEvent::CloseRequested); }
            WinitWindowEvent::Resized(size) => { let _ = self.tx_events.send(WindowEvent::Resized { width: size.width, height: size.height }); }
            WinitWindowEvent::ScaleFactorChanged { scale_factor, .. } => { let _ = self.tx_events.send(WindowEvent::ScaleFactorChanged { scale: scale_factor }); }
            WinitWindowEvent::Focused(f) => { let _ = self.tx_events.send(WindowEvent::FocusChanged { focused: f }); }
            WinitWindowEvent::RedrawRequested => { let _ = self.tx_events.send(WindowEvent::Redraw); }
            _ => {}
        }
    }
}

pub fn start(cfg: WindowConfig) -> Result<WindowHandle> {
    let (tx_events, rx_events) = unbounded::<WindowEvent>();
    let (tx_user, rx_user) = unbounded::<UserMsg>();

    thread::spawn(move || {
        let event_loop = match EventLoop::<UserMsg>::with_user_event().build() {
            Ok(el) => el,
            Err(e) => { eprintln!("Failed to create EventLoop: {e}"); return; }
        };

        let mut app = WindowApp { cfg, tx_events, _rx_user: rx_user, window: None };

        if let Err(e) = event_loop.run_app(&mut app) {
            eprintln!("Event loop exited with error: {e}");
        }
    });

    Ok(WindowHandle { tx_user, rx_events })
}