# bevy_vrm

[![Crates.io](https://img.shields.io/crates/v/bevy_vrm.svg)](https://crates.io/crates/bevy_vrm)
[![CI](https://github.com/unavi-xyz/bevy_vrm/actions/workflows/ci.yml/badge.svg)](https://github.com/unavi-xyz/bevy_vrm/actions/workflows/ci.yml)
![Crates.io](https://img.shields.io/crates/l/bevy_vrm)
[![Documentation](https://docs.rs/bevy_vrm/badge.svg)](https://docs.rs/bevy_vrm)

[Bevy](https://bevyengine.org/) plugin for loading [VRM](https://vrm.dev/en/) avatars.
Aims to support both the VRM 0.0 and VRM 1.0 standards.

## MToon

For the MToon shader, we use the [bevy_shader_mtoon](https://github.com/unavi-xyz/bevy_shader_mtoon) crate.
`bevy_vrm` re-exports it's contents under `bevy_vrm::mtoon`.

![image](https://github.com/unavi-xyz/bevy_vrm/assets/92771507/a10143df-ff3c-4832-9408-bc6f80533c3d)
