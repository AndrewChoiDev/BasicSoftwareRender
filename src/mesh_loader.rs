use obj::*;
use nalgebra::*;
use std::collections::HashSet;
use indexmap::set::IndexSet;
pub struct Mesh {
    pub positions : Vec<Vector3<f32>>,
    pub indices : Vec<usize>,
    pub uvs : Vec<Vector2<f32>>,

}

impl Mesh {
    pub fn new(filename : &str)
    -> Self {
        // Load Objects
        let loaded_obj = Obj::load(filename).unwrap();
        let obj_data = loaded_obj.data;

        let mut unique_obj_indices : IndexSet<[usize ; 2]> = IndexSet::new();
        let mut indices = Vec::new();

        // Every polygon in the obj data
        for poly in 
            obj_data.objects.iter()
            .map(|o| &o.groups).flatten()
            .map(|g| &g.polys).flatten()
            {

            // Indices into the indexed set of unique obj indices
            let mut set_indices = Vec::new();

            // The vertex indices are inserted into the indexed set
            // Whether it's in the set already or not,
            // an index into the indexed set is pushed to the set_indices
            for vertex_indices in 
                poly.0.iter()
                .map(|it| [it.0, it.1.unwrap()]) 
            {
                set_indices.push(unique_obj_indices.insert_full(vertex_indices).0);
            }

            // append the triangle indices from the set indices
            for index in
                (0..(set_indices.len() - 2)).into_iter()
                .flat_map(|j| vec![0, j + 1, j + 2].into_iter())
                .map(|i| set_indices[i])
            {
                indices.push(index);
            }
        }

        let mut positions = Vec::new();
        let mut uvs = Vec::new();

        for indices_value in unique_obj_indices {
            positions.push(obj_data.position[indices_value[0]].into());
            uvs.push(obj_data.texture[indices_value[1]].into());
        }


        Self {
            positions,
            indices,
            uvs,
        }
    }
}