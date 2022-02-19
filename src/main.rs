use std::env;
use serde::{Deserialize};
use std::fs;
use std::io::Write;

#[derive(Deserialize)]
struct Paths {
    user_json_folder_path: String,
    slippi_folder_path: String
}

fn main() -> std::io::Result<()> {
    let paths: Paths = get_folder_paths("filepaths.json");
    let user_json_name = get_user_name_from_args();

    let user_json_file_path = format!("{}/{}.json", paths.user_json_folder_path, user_json_name);
    let new_user_file_path = format!("{}/user.json", paths.slippi_folder_path);

    let new_user_json = std::fs::read_to_string(user_json_file_path).expect(&format!("Could not load user json with name {}", user_json_name));
    create_new_file_with_contents(&new_user_file_path, &new_user_json)
}

fn create_new_file_with_contents(path: &str, contents: &str) -> std::io::Result<()> {
    let mut new_file = std::fs::File::create(path)?;
    new_file.write_all(contents.as_ref())?;
    new_file.sync_data()?;
    Ok(())
}

fn get_folder_paths(json_file_path: &str) -> Paths {
    let file_paths_json = fs::read_to_string(json_file_path).expect("filepaths.json is required and must be found in same directory that the executable is running in");
    let expected_json_format = "{ \"user_json_folder_path\": string, \"slippi_folder_path\": string }";
    let parsing_error_msg = format!("Could not parse from {json_file_path}. Expected format: {expected_json_format}");
    serde_json::from_str(&file_paths_json).expect(&parsing_error_msg)
}

fn get_user_name_from_args() -> String {
    env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("Command line argument containing user name is required. Ex: ./change-slippi-user my-secondary")
        .clone()
}
