#[derive(Clone)]
pub struct TeamMember {
    pub name: String,
    pub skills: Vec<String>,
    pub availability: bool,
    pub workload: u32,
}
