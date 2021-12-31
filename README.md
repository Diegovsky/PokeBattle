# PokeBattle (former name Usuc)
A simple little GUI application to simulate battle between "Pokey Monsters".

## How to run?
Just clone the repo and `cargo run` it!

## What's a pokemon.yaml file?
It's a file where your pokemons will be defined.
Take a look at the `pokemon.yaml` example file this repo provides.

### Cross compiling from Linux to Windows
Since MinGW for some reason isn't able to statically link C++ code,
I made this simple script to compile it, grab a copy of every DLL needed and
create a zip file ready to extract and run:

```bash
chmod +x scripts/windows-release.sh
scripts/windows-release.sh
```

And a new `.zip` file is ready to run on Windows machines.
