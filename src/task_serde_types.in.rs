

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    title: String,
    steps: Vec<Step>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    title: String,
    is_complete: bool,
}
