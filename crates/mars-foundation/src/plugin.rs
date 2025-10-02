use crate::error::Result;

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn build(&self, app: &mut App) -> Result<()>;
}

pub struct App {
    plugins: Vec<Box<dyn Plugin>>,
}

impl App {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn add_plugin<P: Plugin + 'static>(&mut self, plugin: P) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn build(&mut self) -> Result<()> {
        let plugins = std::mem::take(&mut self.plugins);

        for plugin in &plugins {
            plugin.build(self)?;
            tracing::info!("Built plugin: {}", plugin.name());
        }

        self.plugins = plugins;

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        self.build()?;
        tracing::info!("App is running!");
        Ok(())
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
        app.add_plugin(DummyPlugin).run()?;
        Ok(())
    }
}