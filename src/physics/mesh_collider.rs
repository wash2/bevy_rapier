#[cfg(all(feature = "dim3", feature = "render"))]
use crate::{na::Point3, prelude::Real};

#[cfg(all(feature = "dim3", feature = "render"))]
use bevy::{
    render2::mesh::{Mesh, Indices, VertexAttributeValues},
};
#[cfg(all(feature = "dim3", feature = "render"))]
use rapier::math::DIM;
#[cfg(all(feature = "dim3", feature = "render"))]
use std::{convert::TryFrom, error::Error, fmt};

#[cfg(all(feature = "dim3", feature = "render"))]
#[derive(Debug, Clone, Default)]
pub struct VertexFormatError();

#[cfg(all(feature = "dim3", feature = "render"))]
impl fmt::Display for VertexFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid vertex buffer format! Only Float3 is allowed.")
    }
}

#[cfg(all(feature = "dim3", feature = "render"))]
impl Error for VertexFormatError {}

#[cfg(all(feature = "dim3", feature = "render"))]
#[derive(Debug, Clone, Default)]
pub struct VertexPositionAttributeMissing();
#[cfg(all(feature = "dim3", feature = "render"))]

impl fmt::Display for VertexPositionAttributeMissing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vertex position attribute missing.")
    }
}
#[cfg(all(feature = "dim3", feature = "render"))]

impl Error for VertexPositionAttributeMissing {}
#[cfg(all(feature = "dim3", feature = "render"))]
#[derive(Debug, Clone, Default)]
pub struct VertexIndicesMissing();
#[cfg(all(feature = "dim3", feature = "render"))]

impl fmt::Display for VertexIndicesMissing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vertex indices missing.")
    }
}
#[cfg(all(feature = "dim3", feature = "render"))]

impl Error for VertexIndicesMissing {}
#[cfg(all(feature = "dim3", feature = "render"))]
#[derive(Debug, Clone)]
pub enum ErrorSum {
    VertexFormatError(VertexFormatError),
    VertexPositionAttributeMissing(VertexPositionAttributeMissing),
    VertexIndicesMissing(VertexIndicesMissing),
}
#[cfg(all(feature = "dim3", feature = "render"))]

impl Error for ErrorSum {}
#[cfg(all(feature = "dim3", feature = "render"))]

impl fmt::Display for ErrorSum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ErrorSum::VertexFormatError(e) => e.fmt(f),
            ErrorSum::VertexPositionAttributeMissing(e) => e.fmt(f),
            ErrorSum::VertexIndicesMissing(e) => e.fmt(f),
        }
    }
}
#[cfg(all(feature = "dim3", feature = "render"))]

pub struct SharedShapeMesh(pub Mesh);

#[cfg(all(feature = "dim3", feature = "render"))]
// Easy conversion from Bevy Mesh to types used for building SharedShape
impl TryFrom<SharedShapeMesh> for (Vec<Point3<Real>>, Vec<[u32; DIM]>) {
    fn try_from(
        mesh: SharedShapeMesh,
    ) -> Result<(Vec<Point3<Real>>, Vec<[u32; DIM]>), Self::Error> {
        let vertices = mesh.0.attribute(Mesh::ATTRIBUTE_POSITION);
        let indices = mesh.0.indices();

        let vtx: Vec<_> =
            match vertices.ok_or(ErrorSum::VertexPositionAttributeMissing(Default::default()))? {
                VertexAttributeValues::Float32(vtx) => Ok(vtx
                    .chunks(3)
                    .map(|v| Point3::from([v[0] as Real, v[1] as Real, v[2] as Real]))
                    .collect()),
                VertexAttributeValues::Float32x3(vtx) => Ok(vtx
                    .iter()
                    .map(|v| Point3::from([v[0] as Real, v[1] as Real, v[2] as Real]))
                    .collect()),
                _ => Err(ErrorSum::VertexFormatError(Default::default())),
            }?;

        let idx = match indices.ok_or(ErrorSum::VertexIndicesMissing(Default::default()))? {
            Indices::U16(idx) => idx
                .chunks_exact(3)
                .map(|i| [i[0] as u32, i[1] as u32, i[2] as u32])
                .collect(),
            Indices::U32(idx) => idx.chunks_exact(3).map(|i| [i[0], i[1], i[2]]).collect(),
        };

        Ok((vtx, idx))
    }

    type Error = ErrorSum;
}
