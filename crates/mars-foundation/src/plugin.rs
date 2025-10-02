use crate::error::Result;
use crate::app::{App, Stage};

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn build(&self, app: &mut App) -> Result<()>;
}

pub trait AppExt {
    fn add_startup_system(&mut self, f: impl FnMut(&mut App) -> Result<()> + 'static) -> &mut Self;
    fn add_system(&mut self, f: impl FnMut(&mut App) -> Result<()> + 'static) -> &mut Self;
    fn add_render_system(&mut self, f: impl FnMut(&mut App) -> Result<()> + 'static) -> &mut Self;
}

impl AppExt for App {
    fn add_startup_system(&mut self, f: impl FnMut(&mut App) -> Result<()> + 'static) -> &mut Self {
        self.add_system_to_stage(Stage::Startup, f);
        self
    }

    fn add_system(&mut self, f: impl FnMut(&mut App) -> Result<()> + 'static) -> &mut Self {
        self.add_system_to_stage(Stage::Update, f);
        self
    }

    fn add_render_system(&mut self, f: impl FnMut(&mut App) -> Result<()> + 'static) -> &mut Self {
        self.add_system_to_stage(Stage::Render, f);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;

    struct DummyPlugin;
    impl Plugin for DummyPlugin {
        fn name(&self) -> &'static str { "Dummy" }
        fn build(&self, _app: &mut App) -> Result<()> { Ok(()) }
    }

    #[test]
    fn plugin_adds_and_runs() -> Result<()> {
        let mut app = App::new();
        app.add_plugin(DummyPlugin).run_once()?;
        Ok(())
    }
}