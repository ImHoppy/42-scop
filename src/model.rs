use crate::math::{vec2, vec3};
use crate::vertex::Vertex;
use crate::{obj, AppData};
use anyhow::Result;
use std::collections::HashMap;

pub fn load_model(data: &mut AppData, obj_path: String) -> Result<()> {
    let models = obj::load_obj(obj_path)?;

    let mut unique_vertices = HashMap::new();

    for model in &models {
        for index in &model.mesh.indices {
            let pos_offset = (3 * index) as usize;
            let tex_coord_offset = (2 * index) as usize;

            let tex_coord = if model.mesh.tex_coords.len() > 0 {
                vec2(
                    model.mesh.tex_coords[tex_coord_offset],
                    1.0 - model.mesh.tex_coords[tex_coord_offset + 1],
                )
            } else {
                vec2(
                    model.mesh.positions[pos_offset + 1],
                    model.mesh.positions[pos_offset + 2],
                )
            };

            let vertex = Vertex {
                pos: vec3(
                    model.mesh.positions[pos_offset],
                    model.mesh.positions[pos_offset + 1],
                    model.mesh.positions[pos_offset + 2],
                ),
                color: vec3(1.0, 1.0, 1.0),
                tex_coord,
            };

            if let Some(index) = unique_vertices.get(&vertex) {
                data.indices.push(*index as u32);
            } else {
                let index = data.vertices.len();
                unique_vertices.insert(vertex, index);
                data.vertices.push(vertex);
                data.indices.push(index as u32);
            }
        }
    }

    Ok(())
}
