use mars_foundation::prelude::*;

#[test]
fn foundation_smoke_test() -> Result<()> {
    init_logging();

    let v = Vec3::X + Vec3::Y;
    assert_eq!(v, Vec3::new(1.0, 1.0, 0.0));

    let mut app = App::new();
    app.run()?;

    Ok(())
}