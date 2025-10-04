use mars_foundation::{app::App, error::Result};
use mars_window::{WindowPlugin, WindowEvents, WindowEvent, WindowConfig, UserMsg};

mod mock_backend;

#[test]
fn registers_resources_and_drains_events() -> Result<()> {
    let mut app = App::new();

    let (handle_res, harness) = mock_backend::start(WindowConfig::default());
    let handle = handle_res?;

    let plugin = WindowPlugin::with_starter(
        WindowConfig::default(),
        Box::new(move |_| Ok(handle.clone())),
    );
    app.add_plugin(plugin);
    app.build()?;

    assert!(app.get_resource::<WindowEvents>().is_some());

    harness.tx_events.send(WindowEvent::Resized { width: 800, height: 600 }).unwrap();
    harness.tx_events.send(WindowEvent::CloseRequested).unwrap();

    app.run_once()?;

    let collected = {
        let wev = app.get_resource_mut::<WindowEvents>().unwrap();
        wev.drain().collect::<Vec<_>>()
    };

    assert!(matches!(collected[0], WindowEvent::Resized { width: 800, height: 600 }));
    assert!(matches!(collected[1], WindowEvent::CloseRequested));
    Ok(())
}

#[test]
fn render_stage_requests_redraw() -> Result<()> {
    let mut app = App::new();

    let (handle_res, harness) = mock_backend::start(WindowConfig::default());
    let handle = handle_res?;

    let plugin = WindowPlugin::with_starter(
    WindowConfig::default(),
    Box::new(move |_| Ok(handle.clone())),
    );
    app.add_plugin(plugin);
    app.build()?;
    app.run_once()?;

    let mut saw = false;
    while let Ok(msg) = harness.rx_user.try_recv() {
        if matches!(msg, UserMsg::RequestRedraw) {
            saw = true;
            break;
        }
    }
    assert!(saw, "expected a RequestRedraw message during Render");
    Ok(())
}