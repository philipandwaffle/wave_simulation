use bevy::math::Vec3;

pub struct yee_lattice {
    e_field: Array3D<Vec3>,
    h_field: Array3D<Vec3>,
    epsilon: Array3D<Vec3>,
}

pub struct Array3D<T: Copy> {
    v: Vec<T>,
    x_len: usize,
    y_len: usize,
    z_len: usize,
}

impl<T: Copy> Array3D<T> {
    pub fn new(x_len: usize, y_len: usize, z_len: usize) -> Self {
        Self {
            v: Vec::<T>::with_capacity(x_len * y_len * z_len),
            x_len,
            y_len,
            z_len,
        }
    }

    fn xyz_to_index(&self, x: usize, y: usize, z: usize) -> usize {
        x + (y * self.x_len) + (z * self.x_len * self.y_len)
    }

    pub fn get_val(&self, x: usize, y: usize, z: usize) -> T {
        self.v[self.xyz_to_index(x, y, z)]
    }
    pub fn get_val_ref(&self, x: usize, y: usize, z: usize) -> &T {
        &self.v[self.xyz_to_index(x, y, z)]
    }
    pub fn get_val_mut(&mut self, x: usize, y: usize, z: usize) -> &mut T {
        let i = self.xyz_to_index(x, y, z);
        &mut self.v[i]
    }
}
