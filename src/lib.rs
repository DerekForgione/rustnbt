#![allow(unused)]
pub mod family;
pub mod io;
pub(crate) mod table;
pub mod tag;
pub mod macros;
pub mod snbt;
pub mod format;

/// This is the Error type returned from NbtRead and NbtWrite operations that fail.
#[derive(thiserror::Error, Debug)]
pub enum NbtError {
	/// Error from std::io::Error.
	#[error("{0}")]
	IoError(#[from] std::io::Error),
	/// Failure to convert bytes to a UTF-8 string.
	#[error("Failed to read UTF-8 string.")]
	FromUtf8Error(#[from] std::string::FromUtf8Error),
	/// Tag type ID was not recognized, and may be part of an unsupported format.
	#[error("Unsupported Tag ID.")]
	Unsupported{ id_encountered: u8 },
	/// End marker (0x00) was encountered.
	#[error("Encountered the End tag ID marker.")]
	End,
}

// indexmap preserves the insertion order of elements.
// Minecraft does not care what order elements are in, one thing to note
// is that without having insertion order preserved, the order of elements
// in a compound is indeterminable.
// Preserving the order ensures isomorphism between input and output.
#[cfg(feature = "preserve_order")]
use indexmap::IndexMap;
#[cfg(feature = "preserve_order")]
/// The mapping type used for Tag::Compound.
pub type Map = IndexMap<String, tag::Tag>;
// Fallback to HashMap.
#[cfg(not(feature = "preserve_order"))]
/// The mapping type used for Tag::Compound.
pub type Map = std::collections::HashMap<String, tag::Tag>;