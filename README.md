# bevy_vrm

<!-- cargo-rdme start -->

[Bevy](https://bevyengine.org/) plugin for loading [VRM](https://vrm.dev/en/) avatars.
Aims to support both the VRM 0.0 and VRM 1.0 standards.

### Render Layers

[RenderLayers](bevy::render::view::RenderLayers) are used to represent VRM mesh annotations.
If parts of your loaded VRM are invisible, it is likely because of this!

Check out the [layers] module to see the used values.
Add the corresponding layer to your camera for it to be rendered.

<!-- cargo-rdme end -->
