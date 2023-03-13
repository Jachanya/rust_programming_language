pub struct Consumer {
    id: u32,
    first_name: String,
    last_name: String,
    location: String,
}

impl Consumer {
    pub fn new(id: u32, first_name: String, last_name: String, location: String) -> Self{
        Self {
            id: id,
            first_name: first_name,
            last_name: last_name,
            location: location,
        }
    }

    pub fn get_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn get_current_location(&self) { }
    fn get_location(&self) { }
}