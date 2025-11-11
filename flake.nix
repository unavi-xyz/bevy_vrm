{
  inputs = {
    flake-parts = {
      inputs.nixpkgs-lib.follows = "nixpkgs";
      url = "github:hercules-ci/flake-parts";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";

    # Rust
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Other
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs =
    inputs@{ flake-parts, systems, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } (
      { ... }:
      {
        systems = import systems;

        imports = [
          inputs.treefmt-nix.flakeModule
          ./crates/vrm_viewer
        ];

        perSystem =
          {
            config,
            lib,
            pkgs,
            system,
            ...
          }:
          {
            _module.args.pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [
                inputs.fenix.overlays.default

                (
                  self: _:
                  let
                    toolchain = (
                      with self.fenix;
                      combine [
                        complete.toolchain
                        targets.wasm32-unknown-unknown.latest.rust-std
                      ]
                    );
                  in
                  {
                    crane = (inputs.crane.mkLib self).overrideToolchain toolchain;
                  }
                )
              ];
            };

            checks = {
              audit = pkgs.crane.cargoAudit {
                inherit (inputs) advisory-db;
                src = ./.;
                pname = "bevy_vrm";
              };
            };

            treefmt.programs = {
              actionlint.enable = true;
              deadnix.enable = true;
              mdformat.enable = true;
              nixfmt = {
                enable = true;
                strict = true;
              };
              rustfmt.enable = true;
              statix.enable = true;
              taplo.enable = true;
              yamlfmt.enable = true;
            };

            devShells.default = pkgs.crane.devShell {
              packages =
                (with pkgs; [
                  cargo-edit
                  cargo-machete
                  cargo-nextest
                  cargo-rdme
                  cargo-release
                  cargo-workspaces
                ])
                ++ (
                  config.packages
                  |> lib.attrValues
                  |> lib.flip pkgs.lib.forEach (x: x.buildInputs ++ x.nativeBuildInputs)
                );

              LD_LIBRARY_PATH =
                config.packages
                |> lib.attrValues
                |> lib.flip pkgs.lib.forEach (x: x.runtimeDependencies)
                |> lib.concatLists
                |> lib.makeLibraryPath;
            };
          };
      }
    );
}
