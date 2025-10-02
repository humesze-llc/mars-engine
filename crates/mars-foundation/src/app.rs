use crate::error::Result;

pub type SystemFn = Box<dyn FnMut(&mut App) -> Result<()>>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Stage {
    Startup,
    PreUpdate,
    Update,
    PostUpdate,
    Render,
}

pub struct App {
    plugins: Vec<Box<dyn crate::plugin::Plugin>>,
    stages: [Vec<SystemFn>; 5],
    running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            stages: Default::default(),
            running: false,
        }
    }

    pub fn add_plugin<P: crate::plugin::Plugin + 'static>(&mut self, plugin: P) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn add_system_to_stage(&mut self, stage: Stage, f: impl FnMut(&mut App) -> Result<()> + 'static) {
        self.stage_mut(stage).push(Box::new(f));
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn build(&mut self) -> Result<()> {
        let plugins = std::mem::take(&mut self.plugins);
        for p in &plugins {
            p.build(self)?;
        }
        self.plugins = plugins;
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        self.build()?;
        self.running = true;
        while self.running {
            self.run_stage(Stage::PreUpdate)?;
            self.run_stage(Stage::Update)?;
            self.run_stage(Stage::PostUpdate)?;
            self.run_stage(Stage::Render)?;
        }
        Ok(())
    }

    pub fn run_once(&mut self) -> Result<()> {
        self.build()?;
        self.run_stage(Stage::Startup)?;
        self.run_stage(Stage::PreUpdate)?;
        self.run_stage(Stage::Update)?;
        self.run_stage(Stage::PostUpdate)?;
        self.run_stage(Stage::Render)
    }

    fn run_stage(&mut self, stage: Stage) -> Result<()> {
        let mut systems = std::mem::take(self.stage_mut(stage));
        for sys in &mut systems {
            sys(self)?;
        }
        *self.stage_mut(stage) = systems;
        Ok(())
    }

    fn stage_mut(&mut self, stage: Stage) -> &mut Vec<SystemFn> {
        match stage {
            Stage::Startup   => &mut self.stages[0],
            Stage::PreUpdate => &mut self.stages[1],
            Stage::Update    => &mut self.stages[2],
            Stage::PostUpdate=> &mut self.stages[3],
            Stage::Render    => &mut self.stages[4],
        }
    }
}

impl Default for App {
    fn default() -> Self { Self::new() }
}