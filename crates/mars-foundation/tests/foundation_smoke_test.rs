use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use mars_foundation::prelude::*;
use mars_foundation::plugin::AppExt;

#[test]
fn foundation_smoke_test_runs_once() -> Result<()> {
    let ran = Arc::new(AtomicBool::new(false));
    let ran_flag = ran.clone();

    let mut app = App::new();
    app.add_system(move |_app| {
        ran_flag.store(true, Ordering::Relaxed);
        Ok(())
    });

    app.run_once()?;
    assert!(ran.load(Ordering::Relaxed));
    Ok(())
}

#[test]
fn foundation_render_system_runs() -> Result<()> {
    let rendered = Arc::new(AtomicBool::new(false));
    let rendered_flag = rendered.clone();

    let mut app = App::new();
    app.add_render_system(move |_app| {
        rendered_flag.store(true, Ordering::Relaxed);
        Ok(())
    });

    app.run_once()?;
    assert!(rendered.load(Ordering::Relaxed));
    Ok(())
}

#[test]
fn foundation_startup_runs_before_update() -> Result<()> {
    let order: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
    let a = order.clone();
    let b = order.clone();

    let mut app = App::new();
    app.add_startup_system(move |_app| {
        a.lock().unwrap().push("startup");
        Ok(())
    });
    app.add_system(move |_app| {
        b.lock().unwrap().push("update");
        Ok(())
    });

    app.run_once()?;
    let got = order.lock().unwrap().clone();
    assert_eq!(got, vec!["startup", "update"]);
    Ok(())
}