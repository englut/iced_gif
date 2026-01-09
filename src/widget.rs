pub mod gif;

pub use gif::Gif;

/// Creates a new [`Gif`] with the given [`gif::Frames`]
pub fn gif<'a>(frames: &'a gif::Frames) -> Gif<'a> {
    Gif::new(frames)
}
