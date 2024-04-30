pub enum Extension {
    Name(String),
    None,
}

pub struct File {
    pub name: String,
    pub extension: Extension,
    pub permissions: String,
    // pub owner: String,
    // pub group: String,
    // pub size: u32,
    // pub modified_date: String,
    // pub directory: bool,
    // pub executable: bool,
}

impl File {
    pub fn new() -> File {
        return File {
            name: String::new(),
            extension: Extension::None,
            permissions: String::from("----------"),
        };
    }
}

pub fn get_extension(name: String) -> Extension {
    return Extension::None;
}
