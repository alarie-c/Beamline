use std::path::PathBuf;
use super::element::Element;

pub struct Elements {
    elements: Vec<Element>,
}

pub enum Measurement {
    Imperial,
    Metric,
}

pub struct Dimensons {
    width: u8,
    height: u8,
    measurement: Measurement,
}

/// Represents a project and stores all of it's
/// elements' information
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub venue: String,
    dimensions: Dimensons,
    elements: Elements,
}

impl From<Primitive<'_>> for Project {
    /// Conversion from the `Primitive` projct information used in the 
    /// New Project screen to a full `Project` containing empty elements and parsed dimensions
    fn from(pr: Primitive) -> Self {
        let parsed_height: u8 = pr.height.parse().unwrap_or(0);
        let parsed_width: u8 = pr.width.parse().unwrap_or(0);
        
        let measurement = match pr.measurement {
            "ft" => Measurement::Imperial,
            "m" => Measurement::Metric,
            _ => Measurement::Metric,
        };
        
        let dimensions = Dimensons {
            width: parsed_width,
            height: parsed_height,
            measurement,
        };

        let elements = Elements { elements: vec![] };

        Self {
            name: pr.name.to_owned(),
            venue: pr.venue.to_owned(),
            path: pr.path,
            dimensions,
            elements,
        }
    }
}

/// This struct is used to represent the data entered during the
/// new project creation window
/// 
/// This has the basic information for a project and is eventually
/// used to make a full `Project` using `Project::from()`
pub struct Primitive<'new> {
    pub name: &'new str,
    pub venue: &'new str,
    pub path: PathBuf,
    pub height: &'new str,
    pub width: &'new str,
    pub measurement: &'new str,
}