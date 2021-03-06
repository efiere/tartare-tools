# tartare-tools [![Travis Status][travis_badge]][travis] [![Github Actions Status][github_actions_badge]][github_actions]

[travis_badge]: https://img.shields.io/travis/CanalTP/tartare-tools?label=travis
[travis]: https://travis-ci.org/CanalTP/tartare-tools
[github_actions_badge]: https://img.shields.io/github/workflow/status/CanalTP/tartare-tools/Continuous%20Integration?label=gh%20actions
[github_actions]: https://github.com/CanalTP/tartare-tools

more coming soon...

## How to compile Proj version 6
To convert coordinates, Proj library is used. Rust requires a version 6+.

Debian based distributions (even the latest Ubuntu), the distributed version is 5. Therefore this library needs to be compiled.


Make sure you don’t have `libproj-dev` installed and run `make install` to run the compilation locally.


## How to install
First, add `${HOME}/.cargo/bin` to your `PATH`.
```
export PATH=${PATH}:${HOME}/.cargo/bin
```

Then install them with the following command
```
cargo install --path . -f
```

You should then be able to run the binaries with
```
gtfs2ntfs -h
```

## License

Licensed under [GNU General Public License v3.0](LICENSE)
