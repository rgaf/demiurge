mod function;

mod const_node;
mod harmonic_node;
mod hypersphere_node;
mod invert_node;
mod knead_node;
mod lerp_node;
mod multiply_node;
mod overlay_node;
mod perlin_node;
mod screen_node;
mod sigmoid_node;
mod soft_light_node;
mod static_node;
mod tile_node;
mod transform_node;
mod worley_node;

pub use const_node::ConstNode;
pub use harmonic_node::HarmonicNode;
pub use hypersphere_node::HypersphereNode;
pub use invert_node::InvertNode;
pub use knead_node::KneadNode;
pub use lerp_node::LerpNode;
pub use multiply_node::MultiplyNode;
pub use overlay_node::OverlayNode;
pub use perlin_node::PerlinNode;
pub use screen_node::ScreenNode;
pub use sigmoid_node::SigmoidNode;
pub use soft_light_node::SoftLightNode;
pub use static_node::StaticNode;
pub use tile_node::TileNode;
pub use transform_node::TransformNode;
pub use worley_node::{WorleyNode, WorleyPaintMethod};

pub trait NoiseNode<const DIM: usize> {
    fn value_at(&self, point: crate::geometry::RealPoint<DIM>) -> f64;
}
