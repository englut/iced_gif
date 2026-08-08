use std::sync::Arc;

pub mod gif;

pub use gif::Gif;

/// Creates a new [`Gif`] with the given [`gif::Frames`]
pub fn gif(frames: Arc<gif::Frames>, max_loops: Option<usize>) -> Gif {
    Gif::new(frames, max_loops)
}
