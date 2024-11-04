# Calypso 🌍

## What is Calypso?
Calypso is a (WIP) mod loader written by the team at [Nonsensical Dev]() aimed at making it easier than ever for developers to create a quick and simple modding API for their Godot games. Calypso can be easily tailored to fit the developers needs/desires, including options for speed, binary size optimization, custom path configuration, and serveral internal options to play with.

## Why?

Modding has become an integral part of the gaming world, allowing players to customize their experince while playing your game to their hearts content. Calypso's goal is to lower the barrier of entry to growing your games lifespan and more easily fostering a community, simplifying the process for both the developer and end user.

~~And its written in rust 🦀~~


## Getting started

Simply add calypso as a submodule in your `res://addons` folder or download the folder from the [Releases]() tab and place it in the addons folder.

Afterwards restart Godot, and your ready to start using Calypso!

## Usage

As of right now, Calypso is in incredibly early stages. Calypso should NOT be used in any sort of production enviornment whatsoever

Beacause of this, running calypso standalone simply outputs an example `*.cy` file (Calypso module)

## Documentation
WIP. check back later. Or you could [Contribute!]()


## Roadmap

* [ ] Basics
  * Mod loading/unloading ✔️
  * Load resources from mods
  * Provide mods a customizable entry point
  * Prevent mods from using dangerous namespaces
  * Allow customizable mod paths
  * Design basic module specification
  * Basic GDScript API
* [ ] Advanced
    * Mono/.NET Support
    * Dynamic mod asset streaming
    * Support for WASM
    * (Optional) Built in debugging GUI
* [ ] Soon™
  * Add custom bindings support
  * Add OFFICIAL support for lua❓
  * OFFICIAL Mac support (Both runtime and development)
## FAQ
### What platforms does Calypso support?

Calypso is currently in its early stages and primarily targets Windows and Linux platforms. Mac and WASM support is planned for a future release, both for runtime and development environments.

### Is Calypso stable enough for production use?

No, Calypso is in its very early stages and not suitable for production environments. We recommend using it only for testing or development purposes at this point.

### What languages are supported for modding with Calypso?

Right now, Calypso supports GDScript with plans to add support for .NET/Mono, and possibly Lua in future versions.

### Does Calypso support hot-reloading mods?

Currently, hot-reloading is not available, but it’s on our roadmap as we refine mod loading and unloading capabilities.

### What kind of file format does Calypso use for mods? How does Calypso work?

Calypso uses a custom *.cy file format for its modules, which is still under development. More details about this format will be included in future documentation.

### How customizable is Calypso?

Calypso is designed to be flexible, offering developers options for custom paths, binary size optimization, and even support for disabling certain namespaces to maintain security. As Calypso matures, we aim to make customization even more extensive.

### How can I Help?

We’re always open to contributions! Check our Contribute page for details on how to get started. Whether you want to improve documentation, fix bugs, or add new features, your help is welcome.

### Will there be a built-in GUI for managing mods?

A built-in GUI for debugging and managing mods is on our roadmap but is currently planned as an optional feature. For now, mod loading and unloading can be managed programmatically.

## Credits
[@Caznix](https://github.com/Caznix) - Creator of Calypso

[Godot rust](https://godot-rust.github.io/) - Creator of the amazing Godot Rust GDExtension bindings

[Godot Sandbox](https://github.com/libriscv/godot-sandbox) - Inspiration for the creation of Calypso