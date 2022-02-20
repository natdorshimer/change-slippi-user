use std::env;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Default)]
struct SlippiPaths {
    user_json_folder_path: String,
    slippi_folder_path: String
}

fn main() {
    let filepaths_json_path = get_path_of_filepaths_json();
    let paths: SlippiPaths = get_slippi_paths(&filepaths_json_path);

    let user_json_name = get_user_name_from_args();

    let new_user_file_path = PathBuf::from(format!("{}/user.json", paths.slippi_folder_path));
    let user_json_file_path = PathBuf::from(format!("{}/{}.json", paths.user_json_folder_path, user_json_name));

    std::fs::copy(user_json_file_path, new_user_file_path).unwrap();
}

fn get_path_of_filepaths_json() -> Box<Path> {
    let path_of_executable = std::env::current_exe().unwrap();

    path_of_executable
        .parent()
        .unwrap()
        .join("filepaths.json")
        .into_boxed_path()
}

fn get_slippi_paths(json_file_path: &Path) -> SlippiPaths {
    let file_paths_json =
        fs::read_to_string(json_file_path)
            .expect("filepaths.json is required and must be found in same directory that the executable is running in");

    let json_format = serde_json::to_string(&SlippiPaths::default()).unwrap();
    let parsing_error_msg = format!("Could not parse json. Expected format: {json_format}");
    serde_json::from_str(&file_paths_json).expect(&parsing_error_msg)
}

fn get_user_name_from_args() -> String {
    env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("Command line argument for user is required. Ex: `$ ./change-slippi-user my-secondary`")
        .clone()
}
