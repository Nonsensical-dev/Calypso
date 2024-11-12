<div align="center">

# Calypso 🌍
### A Modern Mod Loader for Godot 

[Installation](#usage) • [Docs](#documentation) • [Roadmap](#roadmap) • [Community](#community)

[![Platform - Windows](https://img.shields.io/badge/platform-Windows-blue)](##)
[![Platform - Linux](https://img.shields.io/badge/platform-Linux-blue)](##)
[![Discord](https://img.shields.io/discord/1262066946900361247)](https://discord.gg/kv3jKuPW9F)
[![License](https://img.shields.io/badge/license-MIT-green)]()

<p align="center">
  <sub>🚧 Early Alpha - Not Production Ready 🚧</sub>
</p>

</div>

---

## What is Calypso?
Calypso is a (WIP) mod loader aimed at making it easier than ever for developers to create a quick and simple modding API for their Godot games. Calypso can be easily tailored to fit the developer's needs/desires, including options for speed, binary size optimization, custom path configuration, and several internal options to play with.

## Why?
Modding has become an integral part of the gaming world, allowing players to customize their experience while playing your game to their heart's content. Calypso's goal is to lower the barrier of entry to growing your game's lifespan and more easily fostering a community, simplifying the process for both the developer and end user.

## Quick Start


download Calypso from the [Releases]() tab and place in your `addons` folder.

> 💡 Restart Godot after installation to enable Calypso

## Usage
> ⚠️ **Warning**: As of right now, Calypso is in incredibly early stages. Calypso should NOT be used in any sort of production environment whatsoever.
> 
> Because of this, running Calypso standalone simply outputs an example `*.cy` file (Calypso module).

## Documentation
<div align="center">
  
**[WIP]** Check back later. Or you could [Contribute!]() 📚
  
</div>

## Roadmap

<div align="center">
  
**Current Status**: Implementing Basic Mod Loading ⭐
  
</div>

### 🎯 Basics
- [ ] Mod loading/unloading <- We are here ⭐
- [ ] Basic GDScript API
- [ ] Load resources from mods
- [ ] Provide mods a customizable entry point
- [ ] Prevent mods from using dangerous namespaces
- [ ] Allow customizable mod paths
- [ ] Design basic module specification

### 🚀 Advanced
- [ ] Mono/.NET Support
- [ ] Dynamic mod asset streaming
- [ ] Support for WASM
- [ ] (Optional) Built in debugging GUI

### 🔮 Soon™
- [ ] Add custom bindings support
- [ ] Add OFFICIAL support for Lua❓
- [ ] OFFICIAL Mac support (both runtime and development)

### 🎨 For funsies
- [ ] Official mod manager GUI

## Community/Support

<div align="center">
  
[![Discord Banner](https://img.shields.io/discord/1262066946900361247?style=for-the-badge&logo=discord)](https://discord.gg/kv3jKuPW9F)
  
</div>

## FAQ

<details>
<summary><b>What platforms does Calypso support?</b></summary>

<br>

| Platform | Status |
|----------|---------|
| Windows  | ✅ Supported |
| Linux    | ✅ Supported |
| macOS    | 🔮 Planned |
| WASM     | 🔮 Planned |

</details>

<details>
<summary><b>Is Calypso stable enough for production use?</b></summary>
No, Calypso is in its very early stages and not suitable for production environments. We recommend using it only for testing or development purposes at this point.
</details>

<details>
<summary><b>What languages are supported for modding with Calypso?</b></summary>
Right now, Calypso supports GDScript with plans to add support for .NET/Mono, and possibly Lua in future versions.
</details>

<details>
<summary><b>Does Calypso support hot-reloading mods?</b></summary>
Currently, hot-reloading is not available, but it's on our roadmap as we refine mod loading and unloading capabilities.
</details>

<details>
<summary><b>What kind of file format does Calypso use for mods? How does Calypso work?</b></summary>
Calypso uses a custom `*.cy` file format for its modules based on TOML. More details about this format will be included in future documentation.
</details>

<details>
<summary><b>How customizable is Calypso?</b></summary>
Calypso is designed to be flexible, offering developers options for custom paths, binary size optimization, and even support for disabling certain namespaces to maintain security. As Calypso matures, we aim to make customization even more extensive.
</details>

<details>
<summary><b>How can I help?</b></summary>
We're always open to contributions! Check our Contribute page for details on how to get started. Whether you want to improve documentation, fix bugs, or add new features, your help is welcome.
</details>

<details>
<summary><b>Will there be a built-in GUI for managing mods?</b></summary>
A built-in GUI for debugging and managing mods is on our roadmap but is currently planned as an optional feature. For now, mod loading and unloading can be managed programmatically.
</details>

## Credits

<table>
<tr>
<td align="center">
<a href="https://github.com/Caznix">
<img src="/api/placeholder/100/100" width="100px;" alt="Caznix"/>
<br />
<sub><b>@Caznix</b></sub>
<br />
<sub>Creator</sub>
</a>
</td>
<td align="center">
<a href="https://github.com/libriscv/godot-sandbox">
<img src="/api/placeholder/100/100" width="100px;" alt="Godot Sandbox"/>
<br />
<sub><b>Godot Sandbox</b></sub>
<br />
<sub>Inspiration</sub>
</a>
</td>
</tr>
</table>

---

<div align="center">
  
Made with ❤️ by Nonsensical Dev & the open source community
  
</div>