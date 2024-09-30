use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::SplitWhitespace;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjError {
    OpenFileFailed,
    ParseFailed,
    InvalidMaterialName,
    InvalidObjectName,
    FaceParseError,

    FaceVertexOutOfBounds,
    FaceTexCoordOutOfBounds,
    FaceNormalOutOfBounds,
    InvalidPolygon,
}

#[derive(Clone, Debug)]
pub struct Model {
    pub name: String,
    pub mesh: Mesh,
}

impl Model {
    fn new(name: String, mesh: Mesh) -> Self {
        Model { name, mesh }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Mesh {
    pub positions: Vec<f32>,
    pub vertices: Vec<f32>,
    pub normals: Vec<f32>,
    pub tex_coords: Vec<f32>,
    pub indices: Vec<u32>,
    pub vertex_color: Vec<f32>,
    pub material_id: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct Material {
    pub name: String,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub shininess: f32,
    pub texture: Option<String>,
    pub unknown_param: HashMap<String, String>,
}

/// Some vertices may not have texture coordinates or normals, 0 is used to
/// indicate this as OBJ indices begin at 1
#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Debug, Copy, Clone)]
struct VertexIndices {
    pub v: usize,
    pub vt: usize,
    pub vn: usize,
}

static MISSING_INDEX: usize = usize::MAX;

impl VertexIndices {
    /// Parse the vertex indices from the face string.
    fn parse(
        face_str: &str,
        pos_sz: usize,
        tex_sz: usize,
        norm_sz: usize,
    ) -> Option<VertexIndices> {
        let mut indices = [MISSING_INDEX; 3];
        for i in face_str.split('/').enumerate() {
            // Catch case of v//vn where we'll find an empty string in one of our splits
            // since there are no texcoords for the mesh.
            if !i.1.is_empty() {
                match isize::from_str_radix(i.1, 10) {
                    Ok(x) => {
                        // Handle relative indices
                        *indices.get_mut(i.0)? = if x < 0 {
                            match i.0 {
                                0 => (pos_sz as isize + x) as _,
                                1 => (tex_sz as isize + x) as _,
                                2 => (norm_sz as isize + x) as _,
                                _ => return None, // Invalid number of elements for a face
                            }
                        } else {
                            (x - 1) as _
                        };
                    }
                    Err(_) => return None,
                }
            }
        }
        Some(VertexIndices {
            v: indices[0],
            vt: indices[1],
            vn: indices[2],
        })
    }
}

/// Enum representing a face, storing indices for the face vertices.
#[derive(Debug)]
enum Face {
    Point(VertexIndices),
    Line(VertexIndices, VertexIndices),
    Triangle(VertexIndices, VertexIndices, VertexIndices),
    Quad(VertexIndices, VertexIndices, VertexIndices, VertexIndices),
    Polygon(Vec<VertexIndices>),
}

pub fn load_mtl<F>(file_name: F) -> Result<(Vec<Material>, HashMap<String, usize>), ObjError>
where
    F: AsRef<Path> + std::fmt::Debug,
{
    let file = File::open(file_name.as_ref()).map_err(|error| {
        log::error!("Failed to open file {:?} due to {}", file_name, error);
        ObjError::OpenFileFailed
    })?;
    let reader = BufReader::new(file);

    let mut materials = Vec::new();
    let mut mat_map: HashMap<String, usize> = HashMap::new();
    let mut current_mat: Option<Material> = None;

    for line in reader.lines() {
        let (line, mut words) = match line {
            Ok(ref line) => (line.trim(), line.split_whitespace()),
            Err(err) => {
                log::error!("Failed to read line due to {}", err);
                return Err(ObjError::ParseFailed);
            }
        };
        match words.next() {
            Some("#") | None => continue,
            Some("mtllib") => {
                if let Some(mat) = current_mat.to_owned() {
                    mat_map.insert(mat.name.clone(), materials.len());
                    materials.push(mat);
                }
                let mut mat = Material::default();
                mat.name = line["mtllib".len()..].trim().to_owned();
                if mat.name.is_empty() {
                    return Err(ObjError::InvalidMaterialName);
                }
                current_mat = Some(mat);
            }
            Some(unknown) => {
                if !unknown.is_empty() {
                    let param = line[unknown.len()..].trim().to_owned();
                    if let Some(ref mut mat) = current_mat {
                        mat.unknown_param.insert(unknown.to_owned(), param);
                    }
                } else {
                    log::warn!("Unknown line: {}", line);
                }
            }
        }
    }
    Ok((materials, mat_map))
}

fn parse_vertex_data(
    words: &mut std::str::SplitWhitespace,
    target: &mut Vec<f32>,
    size: usize,
    line: &str,
    log_prefix: &str,
) {
    let old_len = target.len();
    for value in words.by_ref().take(size) {
        target.push(value.parse().unwrap_or_else(|_| {
            log::warn!("Invalid {} vertex: {}", log_prefix, line);
            f32::default()
        }));
    }
    if target.len() - old_len != size {
        log::warn!("Invalid {} vertex: {}", log_prefix, line);
        target.truncate(old_len);
    }
}

/// Parse vertex indices for a face and append it to the list of faces passed.
///
/// Returns `false` if an error occured parsing the face.
fn parse_face(
    face_str: SplitWhitespace,
    faces: &mut Vec<Face>,
    pos_sz: usize,
    tex_sz: usize,
    norm_sz: usize,
) -> bool {
    let mut indices = Vec::new();
    for f in face_str {
        match VertexIndices::parse(f, pos_sz, tex_sz, norm_sz) {
            Some(v) => indices.push(v),
            None => return false,
        }
    }
    // Check what kind face we read and push it on
    match indices.len() {
        1 => faces.push(Face::Point(indices[0])),
        2 => faces.push(Face::Line(indices[0], indices[1])),
        3 => faces.push(Face::Triangle(indices[0], indices[1], indices[2])),
        4 => faces.push(Face::Quad(indices[0], indices[1], indices[2], indices[3])),
        _ => faces.push(Face::Polygon(indices)),
    }
    true
}

/// Add a vertex to a mesh by either re-using an existing index (e.g. it's in
/// the `index_map`) or appending the position, texcoord and normal as
/// appropriate and creating a new vertex.
fn add_vertex(
    mesh: &mut Mesh,
    index_map: &mut HashMap<VertexIndices, u32>,
    vert: &VertexIndices,
    pos: &[f32],
    normal: &[f32],
    tex_coord: &[f32],
) -> Result<(), ObjError> {
    match index_map.get(vert) {
        Some(&i) => mesh.indices.push(i),
        None => {
            let v = vert.v;
            if v.saturating_mul(3).saturating_add(2) >= pos.len() {
                return Err(ObjError::FaceVertexOutOfBounds);
            }
            // Add the vertex to the mesh
            mesh.positions.push(pos[v * 3]);
            mesh.positions.push(pos[v * 3 + 1]);
            mesh.positions.push(pos[v * 3 + 2]);
            if !tex_coord.is_empty() && vert.vt != MISSING_INDEX {
                let vt = vert.vt;
                if vt * 2 + 1 >= tex_coord.len() {
                    return Err(ObjError::FaceTexCoordOutOfBounds);
                }
                mesh.tex_coords.push(tex_coord[vt * 2]);
                mesh.tex_coords.push(tex_coord[vt * 2 + 1]);
            }
            if !normal.is_empty() && vert.vn != MISSING_INDEX {
                let vn = vert.vn;
                if vn * 3 + 2 >= normal.len() {
                    return Err(ObjError::FaceNormalOutOfBounds);
                }
                mesh.normals.push(normal[vn * 3]);
                mesh.normals.push(normal[vn * 3 + 1]);
                mesh.normals.push(normal[vn * 3 + 2]);
            }
            let next = index_map.len() as u32;
            mesh.indices.push(next);
            index_map.insert(*vert, next);
        }
    }
    Ok(())
}

/// Export a list of faces to a mesh.
fn export_faces(
    pos: &[f32],
    tex_coords: &[f32],
    normal: &[f32],
    faces: &[Face],
    material_id: Option<usize>,
) -> Result<Mesh, ObjError> {
    let mut index_map: HashMap<VertexIndices, u32> = HashMap::new();
    let mut mesh = Mesh {
        material_id,
        ..Default::default()
    };

    for face in faces {
        match *face {
            Face::Point(_) => {
                log::warn!("Point faces are not supported");
            },
            Face::Line(_, _) => {
                log::warn!("Line faces are not supported");
            },
            Face::Triangle(ref a, ref b, ref c) => {
                add_vertex(&mut mesh, &mut index_map, a, pos, normal, tex_coords)?;
                add_vertex(&mut mesh, &mut index_map, b, pos, normal, tex_coords)?;
                add_vertex(&mut mesh, &mut index_map, c, pos, normal, tex_coords)?;
            },
            Face::Quad(ref a, ref b, ref c, ref d) => {
                add_vertex(&mut mesh, &mut index_map, a, pos, normal, tex_coords)?;
                add_vertex(&mut mesh, &mut index_map, b, pos, normal, tex_coords)?;
                add_vertex(&mut mesh, &mut index_map, c, pos, normal, tex_coords)?;

                add_vertex(&mut mesh, &mut index_map, a, pos, normal, tex_coords)?;
                add_vertex(&mut mesh, &mut index_map, c, pos, normal, tex_coords)?;
                add_vertex(&mut mesh, &mut index_map, d, pos, normal, tex_coords)?;
            },
            Face::Polygon(ref indices) => {
                let mut iter = indices.iter();
                let first = iter.next().unwrap();
                let second = iter.next().unwrap();
                for vert in iter {
                    add_vertex(&mut mesh, &mut index_map, first, pos, normal, tex_coords)?;
                    add_vertex(&mut mesh, &mut index_map, second, pos, normal, tex_coords)?;
                    add_vertex(&mut mesh, &mut index_map, vert, pos, normal, tex_coords)?;
                }

                let a = indices.first().ok_or(ObjError::InvalidPolygon)?;
                let mut b = indices.get(1).ok_or(ObjError::InvalidPolygon)?;
                for c in indices.iter().skip(2) {
                    add_vertex(&mut mesh, &mut index_map, a, pos, normal, tex_coords)?;
                    add_vertex(&mut mesh, &mut index_map, b, pos, normal, tex_coords)?;
                    add_vertex(&mut mesh, &mut index_map, c, pos, normal, tex_coords)?;
                    b = c;
                }

            },
        }
    }

    Ok(mesh)
}

// Follow the Wavefront .obj file format specification (https://paulbourke.net/dataformats/obj/)
pub fn load_obj<F>(file_name: F) -> Result<Vec<Model>, ObjError>
where
    F: AsRef<Path> + std::fmt::Debug,
{
    let file = File::open(file_name.as_ref()).map_err(|error| {
        log::error!("Failed to open file {:?} due to {}", file_name, error);
        ObjError::OpenFileFailed
    })?;
    let reader = BufReader::new(file);

    // let mut materials = Vec::new();
    let mut models: Vec<Model> = Vec::new();

    let mut current_name = "undefined".to_owned();

    let mut current_pos: Vec<f32> = Vec::new();
    let mut current_normals: Vec<f32> = Vec::new();
    let mut current_tex_coords: Vec<f32> = Vec::new();
    let mut current_faces: Vec<Face> = Vec::new();

    for line in reader.lines() {
        let (line, mut words) = match line {
            Ok(ref line) => (line.trim(), line.split_whitespace()),
            Err(err) => {
                log::error!("Failed to read line due to {}", err);
                return Err(ObjError::ParseFailed);
            }
        };
        match words.next() {
            Some("#") | None => continue,
            Some("v") => parse_vertex_data(&mut words, &mut current_pos, 3, line, "position"),
            Some("vn") => parse_vertex_data(&mut words, &mut current_normals, 3, line, "normal"),
            Some("vt") => {
                parse_vertex_data(&mut words, &mut current_tex_coords, 2, line, "texture")
            }
            Some("f") | Some("l") => {
                if !parse_face(
                    words,
                    &mut current_faces,
                    current_pos.len() / 3,
                    current_normals.len() / 3,
                    current_tex_coords.len() / 2,
                ) {}
                return Err(ObjError::FaceParseError);
            }
            Some("o") | Some("g") => {
                if !current_faces.is_empty() {
                    models.push(Model::new(
                        current_name,
                        export_faces(
                            &current_pos,
                            &current_tex_coords,
                            &current_normals,
                            &current_faces,
                            None,
                        )?,
                    ));
                    current_faces.clear();
                }
                let size = line.chars().next().unwrap().len_utf8();
                current_name = line[size..].trim().to_owned();
                if current_name.is_empty() {
                    current_name = "undefined".to_owned();
                }
            }
            Some("mtllib") => {
                log::trace!("mtllib not implemented");
            }
            Some(_) => {
                log::warn!("Unknown line: {}", line);
            }
        }
    }
    Ok(models)
}
