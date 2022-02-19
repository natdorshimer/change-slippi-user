Little tool so you can swap out user jsons easily

To run:

`$ cargo run user-name` inside of `src/`
or you can build and use the executable

It will look for `user-name.json` inside of the folder set by `user_json_folder_path`. Then it'll place it in the folder marked by `slippi_folder_path` and change the name to `user.json`.