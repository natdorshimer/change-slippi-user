Little tool so you can swap out user jsons easily

To run:

`$ cargo run user-name` inside of root directory.

Or you can build and use the executable with `$ cargo build --release` as long as the json is in the same folder as the executable.

It will look for `user-name.json` inside of the folder set by `user_json_folder_path`. Then it'll place it in the folder marked by `slippi_folder_path` and change the name to `user.json`.


Make sure to configure your filepaths.json to your actual folders, mine may not match up with yours 

```json
{
  "user_json_folder_path": "C:/games/user-jsons",
  "slippi_folder_path": "C:/games/melee-dolphin"
}
```

```
└── C:/
    └── games/
        ├── user-jsons/
        │   ├── fox.json
        │   ├── falco.json
        │   ├── marth.json
        │   └── smurf.json
        └── melee-dolphin/
            ├── Slippi Dolphin.exe
            └── user.json
```
