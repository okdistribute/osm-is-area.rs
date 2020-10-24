//! Returns true if the given way is an area according to [Overpass turbo](https://wiki.openstreetmap.org/wiki/Overpass_turbo/Polygon_Features)
//! 
//! ## Examples 
//! ```
//! use osm_is_area;
//!
//! let tags = vec![
//!  (r"waterway", r"riverbank")
//! ];
//! let refs = vec![1, 3, 2, 1];
//!
//! let is_area = osm_is_area::way(&tags, &refs);
//! assert_eq!(true, is_area);
//! ```
//!
//! A relation is an area when it has a tag "type" with value "multipolygon".
//! ```
//! use osm_is_area;
//!
//! let tags = vec![
//!     (r"type", r"multipolygon")
//! ];
//! let members = vec![1, 3, 2, 1];
//!
//! let is_area = osm_is_area::relation(&tags, &members);
//! assert_eq!(true, is_area);
//! ```
#[doc(inline)]
mod way;
pub use way::*;

mod relation;
pub use relation::*;

pub mod polygon_features;

