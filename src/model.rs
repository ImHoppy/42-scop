use crate::math::{vec2, vec3};
use crate::vertex::Vertex;
use crate::{obj, AppData};
use anyhow::Result;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::BufReader;

pub fn load_model(data: &mut AppData) -> Result<()> {
    let models = obj::load_obj("./resources/texture_cube.obj")?;

    for model in &models {
        for index in &model.mesh.indices {
            let pos_offset = (3 * index) as usize;
            let tex_coord_offset = (2 * index) as usize;

            let vertex = Vertex {
                pos: vec3(
                    model.mesh.positions[pos_offset],
                    model.mesh.positions[pos_offset + 1],
                    model.mesh.positions[pos_offset + 2],
                ),
                color: vec3(1.0, 1.0, 1.0),
                tex_coord: vec2(
					model.mesh.tex_coords[tex_coord_offset],
					1.0 - model.mesh.tex_coords[tex_coord_offset + 1],
                ),
            };
            data.vertices.push(vertex);
            data.indices.push(data.indices.len() as u32);
        }
    }

    Ok(())
}
