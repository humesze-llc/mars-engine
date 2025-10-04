use mars_foundation::{error::Result, app::{App, Stage}};
use mars_foundation::plugin::Plugin;
use crate::{WindowConfig, WindowEvents, WindowEvent, UserMsg, WindowHandle, start};

pub struct WindowPlugin {
    pub config: WindowConfig,
    starter: Box<dyn Fn(WindowConfig) -> Result<WindowHandle> + Send + Sync + 'static>,
}

impl Default for WindowPlugin {
    fn default() -> Self {
        Self {
            config: WindowConfig::default(),
            starter: Box::new(start),
        }
    }
}

impl WindowPlugin {
    pub fn with_starter(
        config: WindowConfig,
        starter: Box<dyn Fn(WindowConfig) -> Result<WindowHandle> + Send + Sync + 'static>,
    ) -> Self {
        Self { config, starter }
    }
}

impl Plugin for WindowPlugin {
    fn name(&self) -> &'static str { "WindowPlugin" }

    fn build(&self, app: &mut App) -> Result<()> {
        let handle = (self.starter)(self.config.clone())?;
        let event_rx = handle.rx_events.clone();
        let user_tx = handle.tx_user.clone();

        app.insert_resource(handle);
        app.insert_resource(WindowEvents::new());

        app.add_system_to_stage(Stage::PreUpdate, move |app| {
            if let Some(wev) = app.get_resource_mut::<WindowEvents>() {
                while let Ok(ev) = event_rx.try_recv() {
                    wev.push(ev);
                }
            }
            Ok(())
        });

        app.add_system_to_stage(Stage::Render, move |app| {
            if let Some(wev) = app.get_resource_mut::<WindowEvents>() {
                wev.push(WindowEvent::Redraw);
            }
            let _ = user_tx.send(UserMsg::RequestRedraw);
            Ok(())
        });

        Ok(())
    }
}