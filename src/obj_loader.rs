use crate::bvh::BVHNode;
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::triangle::Triangle;
use crate::vec3::Point3;
use std::sync::Arc;
use tobj;

pub fn load_obj(file_name: &str, mat: Arc<dyn Material>) -> BVHNode {
    let options = tobj::LoadOptions {
        triangulate: true,
        single_index: true,
        ..Default::default()
    };
    match tobj::load_obj(file_name, &options) {
        Ok((models, _)) => {
            let mut meshes = HittableList::new();

            eprintln!("Num models: {}", models.len());
            for m in models {
                eprintln!("Loading model {}", m.name);

                let mesh = m.mesh;

                let positions: Vec<Point3> = mesh
                    .positions
                    .chunks(3)
                    // .map(|i| Point3::new(i[0] as f64, i[1] as f64, i[2] as f64))
                    .map(|i| {
                        Point3::new(
                            i[0] as f64 * 100.0,
                            i[1] as f64 * 100.0,
                            i[2] as f64 * 100.0,
                        )
                    })
                    .collect();

                for tri in mesh.indices.chunks(3) {
                    let v0 = positions[tri[0] as usize];
                    let v1 = positions[tri[1] as usize];
                    let v2 = positions[tri[2] as usize];

                    meshes.add(Box::new(Triangle::new(v0, v2, v1, mat.clone())));
                }
            }

            return BVHNode::from_list(meshes);
        }
        Err(e) => {
            eprintln!("Failed to load {:?} due to {:?}", file_name, e);
            // return HittableList::new();
            return BVHNode::from_list(HittableList::new());
        }
    }
}
