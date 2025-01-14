# MeowList - GTK Todo List.
------------
MeowList makes task management easy, simple and reproducable. With a simple GUI, and easy to migrate database you can produce task lists on all your devices, quickly, easily and effectively.<br>
------------
## Installing MeowList
MeowList has multiple methods of installation, depending on what operating system you are on, you'll want to follow a specific method.<br>
### Apple Silicon macOS Devices
For Apple Silicon macOS Device's you can simply download the .dmg file and install the app to your Applications directory, with no additional dependencies needed. If the app is broken, you may need to allow it past your firewall. To do this you should be able to go into your system privacy settings, and allow the app past firewall.
### Windows, macOS (Intel) and Linux
The process for these devices is much more complicated than the Apple Silicon approach. To begin you need to install RustUp, which can be found at [RustUp](https://rustup.rs/), you then need to clone the repository, hoping you have Git installed, if you don't you can just download the actual repository to downloads and CD into that. You can then, simply build the application with `cargo build` and then `cargo install`, if you'd like to test it before installing run `cargo run`. Elsewise you can just do `meowlist` in your terminal after installing.<br>

## Using MeowList for repoducable notes.
Meowlist stores data in SQLite databases, making it very simple to find, and copy a todo list over to another person. Depending on your OS it may be in a different place. If you are on a macOS System, your database will be in `~/Application Support/meowlist`, if you are on Linux it will be in `~/.local/share/meowlist` and on Windows it should be in `%appdata%/.local/meowlist` (Untested), if you wish to use someone elses data, just override your data in those file locations. Be sure to rename your database and back it up! Meowlist ALWAYS reads from a database called tasks.
