use std::{fs, io};

use toml::Value;

pub struct Settings {
    settings_toml: String,
}

impl Settings {
    /// Creates a new instance of the Settings struct
    pub fn new() -> Result<Self, io::Error> {
        const SETTINGS_PATH: &str = "data/settings.toml";
        match fs::read_to_string(SETTINGS_PATH) {
            Ok(settings_toml) => Ok(Self { settings_toml }),
            Err(e) => Err(e),
        }
    }
}

pub struct ProjectPrimitive {
    pub name: String,
    pub venue: String,
    pub path: String,
    pub size: (String, String),
    pub dimensions: String,
}

impl ProjectPrimitive {
    pub fn new_blank_for_project_creator() -> Self {
        Self {
            name: String::new(),
            venue: String::new(),
            path: String::new(),
            size: (String::from("20"), String::from("20")),
            dimensions: String::from("ft"),
        }
    }
}

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub venue: String,
    pub path: String,
    pub size: (u8, u8),
    pub data: String,
}

impl From<ProjectPrimitive> for Project {
    fn from(value: ProjectPrimitive) -> Self {
        Self {
            name: value.name,
            venue: value.venue,
            path: value.path.clone(),
            size: (
                value.size.0.parse::<u8>().unwrap_or(0),
                value.size.1.parse::<u8>().unwrap_or(0),
            ),
            data: fs::read_to_string(value.path).unwrap_or(String::new()),
        }
    }
}

impl Project {
    // pub fn new_blank_for_project_creator() -> Self {
    //     Self {
    //         name: String::new(),
    //         venue: String::new(),
    //         path: String::new(),
    //         data: String::new(),
    //     }
    // }

    pub fn try_from_path(path: &str) -> Option<Self> {
        match fs::read_to_string(path) {
            Ok(data) => {
                let name = Self::get_name_from_data(&data);
                let venue = Self::get_venue_from_data(&data);
                let height = Self::get_height_from_data(&data);
                let width = Self::get_width_from_data(&data);

                if name.is_none() || venue.is_none() || height.is_none() || width.is_none() {
                    return None;
                } else {
                    return Some(Self {
                        name: name.unwrap(),
                        venue: venue.unwrap(),
                        path: path.to_owned(),
                        size: (
                            height.unwrap().parse::<u8>().unwrap_or(0),
                            width.unwrap().parse::<u8>().unwrap_or(0),
                        ),
                        data,
                    });
                }
            }
            Err(_) => return None,
        }
    }

    fn get_name_from_data(data: &String) -> Option<String> {
        let parsed: Value = data.parse().expect("Error reading .beamline");
        match parsed.get("header").and_then(|header| header.get("name")) {
            Some(name) => match name {
                Value::String(s) => return Some(s.to_owned()),
                _ => return None,
            },
            None => return None,
        }
    }

    fn get_venue_from_data(data: &String) -> Option<String> {
        let parsed: Value = data.parse().expect("Error reading .beamline");
        match parsed.get("header").and_then(|header| header.get("venue")) {
            Some(venue) => match venue {
                Value::String(s) => return Some(s.to_owned()),
                _ => return None,
            },
            None => return None,
        }
    }

    fn get_height_from_data(data: &String) -> Option<String> {
        let parsed: Value = data.parse().expect("Error reading .beamline");
        match parsed.get("header").and_then(|header| header.get("height")) {
            Some(height) => match height {
                Value::String(s) => return Some(s.to_owned()),
                _ => return None,
            },
            None => return None,
        }
    }

    fn get_width_from_data(data: &String) -> Option<String> {
        let parsed: Value = data.parse().expect("Error reading .beamline");
        match parsed.get("header").and_then(|header| header.get("width")) {
            Some(width) => match width {
                Value::String(s) => return Some(s.to_owned()),
                _ => return None,
            },
            None => return None,
        }
    }
}

#[derive(Debug)]
pub struct Projects {
    /// Stores all recently opened projects that are tracked in settings.toml
    pub projects: Vec<Project>,
}

impl Projects {
    pub fn new(settings: &Settings) -> Option<Self> {
        let mut stack_projects = Vec::<Project>::new();

        let parsed: Value = settings
            .settings_toml
            .parse()
            .expect("Error reading settings.toml");

        match parsed
            .get("projects")
            .and_then(|projects| projects.get("recent"))
        {
            Some(list) => match list {
                Value::Array(array) => {
                    array.into_iter().for_each(|entry| {
                        let p = Project::try_from_path(
                            entry
                                .as_str()
                                .expect("Recently opened project entry is not a string!"),
                        );
                        if p.is_some() {
                            stack_projects.push(p.unwrap())
                        }
                    });
                    return Some(Self {
                        projects: stack_projects,
                    });
                }
                _ => return None,
            },
            None => {
                return None;
            }
        }
    }
}
