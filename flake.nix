{
  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (localSystem:
      let
        pkgs = import nixpkgs {
          inherit localSystem;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        rustToolchain =
          pkgs.pkgsBuildHost.rust-bin.stable.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        commonArgs = {
          src = lib.cleanSource ./.;

          strictDeps = true;

          buildInputs = lib.optionals pkgs.stdenv.isLinux (with pkgs; [
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
          ]) ++ lib.optionals pkgs.stdenv.isDarwin
            (with pkgs; [ pkgs.darwin.apple_sdk.frameworks.Cocoa ]);

          nativeBuildInputs = with pkgs;
            [ nodePackages.prettier pkg-config ]
            ++ lib.optionals (!pkgs.stdenv.isDarwin)
            (with pkgs; [ alsa-lib alsa-lib.dev ]);
        };

        commonShell = {
          checks = self.checks.${localSystem};
          packages = with pkgs; [ cargo-rdme cargo-watch rust-analyzer ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath commonArgs.buildInputs;
        };

        cargoArtifacts =
          craneLib.buildDepsOnly (commonArgs // { pname = "deps"; });

        cargoArtifactsWasm = craneLib.buildDepsOnly (commonArgs // {
          pname = "deps-wasm";
          doCheck = false;
        });

        cargoClippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          pname = "clippy";
        });

        cargoDoc = craneLib.cargoDoc (commonArgs // {
          inherit cargoArtifacts;
          pname = "doc";
        });

        bevy_shader_mtoon = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "bevy_shader_mtoon";
          cargoExtraArgs = "-p bevy_shader_mtoon";
        });

        bevy_vrm = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "bevy_vrm";
          cargoExtraArgs = "-p bevy_vrm";
        });
      in {
        checks = { inherit bevy_vrm bevy_shader_mtoon cargoClippy cargoDoc; };

        apps = {
          generate-readme = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "generate-readme" ''
              cd crates

              for folder in */; do
                (cd $folder && cargo rdme)
              done
            '';
          };
        };

        packages = {
          bevy_shader_mtoon = bevy_shader_mtoon;
          bevy_vrm = bevy_vrm;

          default = pkgs.symlinkJoin {
            name = "all";
            paths = [ bevy_shader_mtoon bevy_vrm ];
          };
        };

        devShells.default = craneLib.devShell commonShell;
      });
}
