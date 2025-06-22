{
  inputs = {
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    treefmt-nix.url = "github:numtide/treefmt-nix";

  };

  outputs =
    {
      crane,
      flake-utils,
      nixpkgs,
      rust-overlay,
      self,
      treefmt-nix,

      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      localSystem:
      let
        pkgs = import nixpkgs {
          inherit localSystem;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        commonArgs = {
          src = lib.fileset.toSource rec {
            root = ./.;
            fileset = lib.fileset.unions [
              (craneLib.fileset.commonCargoSources root)
              (lib.fileset.fileFilter (
                file:
                lib.any file.hasExt [
                  "html"
                  "wgsl"
                ]
              ) root)
              ./assets
              ./public
            ];
          };

          strictDeps = true;

          buildInputs =
            lib.optionals pkgs.stdenv.isLinux (
              with pkgs;
              [
                alsa-lib
                alsa-lib.dev
                libxkbcommon
                udev
                vulkan-loader
                wayland
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                xorg.libXrandr
              ]
            )
            ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs; [ darwin.apple_sdk.frameworks.Cocoa ]);

          nativeBuildInputs =
            (with pkgs; [
              binaryen
              pkg-config
              trunk
              wasm-bindgen-cli

            ])
            ++ lib.optionals (!pkgs.stdenv.isDarwin) (
              with pkgs;
              [
                alsa-lib
                alsa-lib.dev
              ]
            );
        };

        commonShell = {
          checks = self.checks.${localSystem};
          packages = with pkgs; [
            cargo-edit
            cargo-rdme
            cargo-release
            cargo-watch
            cargo-workspaces
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath commonArgs.buildInputs;
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // { pname = "deps"; });

        cargoClippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "clippy";
          }
        );

        cargoDoc = craneLib.cargoDoc (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "doc";
          }
        );

        bevy_shader_mtoon = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "bevy_shader_mtoon";
            cargoExtraArgs = "-p bevy_shader_mtoon";
          }
        );

        bevy_vrm = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "bevy_vrm";
            cargoExtraArgs = "-p bevy_vrm";
          }
        );

        gltf_kun_vrm = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "gltf_kun_vrm";
            cargoExtraArgs = "-p gltf_kun_vrm";
          }
        );

        serde_vrm = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            pname = "serde_vrm";
            cargoExtraArgs = "-p serde_vrm";
          }
        );

        vrm_viewer = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;

            pname = "vrm_viewer";
            cargoExtraArgs = "-p vrm_viewer";
            postInstall = ''
              cp -r assets $out/bin/
            '';
          }
        );

        vrm_viewer_web = craneLib.buildTrunkPackage (
          commonArgs
          // {
            pname = "vrm_viewer_web";
            cargoExtraArgs = "-p vrm_viewer";

            wasm-bindgen-cli = pkgs.wasm-bindgen-cli;
          }
        );

        treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
      in
      {
        formatter = treefmtEval.config.build.wrapper;

        checks = {
          inherit
            bevy_vrm
            bevy_shader_mtoon
            gltf_kun_vrm
            serde_vrm
            vrm_viewer
            vrm_viewer_web
            cargoClippy
            cargoDoc
            ;
        };

        apps = {
          vrm_viewer = flake-utils.lib.mkApp { drv = vrm_viewer; };
          vrm_viewer_web = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "vrm_viewer_web" ''
              ${pkgs.python3Minimal}/bin/python3 -m http.server --directory ${
                self.packages.${localSystem}.vrm_viewer_web
              } 3000
            '';
          };
        };

        packages = {
          bevy_shader_mtoon = bevy_shader_mtoon;
          bevy_vrm = bevy_vrm;
          gltf_kun_vrm = gltf_kun_vrm;
          serde_vrm = serde_vrm;
          vrm_viewer = vrm_viewer;
          vrm_viewer_web = vrm_viewer_web;

          default = pkgs.symlinkJoin {
            name = "viewer";
            paths = [
              vrm_viewer
              vrm_viewer_web
            ];
          };
        };

        devShells.default = craneLib.devShell commonShell;
      }
    );
}
