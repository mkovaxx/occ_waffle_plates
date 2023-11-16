use glam::DVec3;
use opencascade::{primitives::IntoShape, workplane::Workplane};

fn main() -> anyhow::Result<()> {
    let block = Workplane::xy()
        .rect(225.0, 132.0)
        .to_face()
        .extrude(DVec3::new(0.0, 0.0, 22.0));

    block.into_shape().write_stl("waffle_plate.stl")?;

    Ok(())
}
