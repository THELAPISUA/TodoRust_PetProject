use std::fs::File;
use std::io::BufWriter;

use crate::models::task::Task;

pub fn load(file_path: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(file_path)?;
    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}
pub fn save(tasks: &Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("save.json")?;
    let writer = BufWriter::new(file);

    serde_json::to_writer(writer, tasks)?;

    Ok(())
}
