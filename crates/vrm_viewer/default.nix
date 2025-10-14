_: {
  perSystem =
    { pkgs, lib, ... }:
    let
      pname = "vrm_viewer";

      buildInputs = pkgs.lib.optionals pkgs.stdenv.isLinux (
        with pkgs;
        [
          alsa-lib
          libxkbcommon
          openssl
          udev
          vulkan-loader
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ]
      );

      nativeBuildInputs = pkgs.lib.optionals pkgs.stdenv.isLinux (with pkgs; [ pkg-config ]);

      src = lib.fileset.toSource rec {
        root = ../..;
        fileset = lib.fileset.unions [
          (pkgs.crane.fileset.commonCargoSources root)
          (lib.fileset.fileFilter (
            file:
            lib.any file.hasExt [
              "html"
              "wgsl"
            ]
          ) root)
          ../../LICENSE-APACHE
          ../../LICENSE-MIT
          ../../assets
          ../../public
        ];
      };

      cargoArgs = {
        inherit buildInputs;
        inherit nativeBuildInputs;
        inherit pname;
        inherit src;

        runtimeDependencies = buildInputs;

        cargoExtraArgs = "-p ${pname}";
        strictDeps = true;
      };

      cargoArtifacts = pkgs.crane.buildDepsOnly cargoArgs;
    in
    {
      checks = {
        "${pname}-doc" = pkgs.crane.cargoDoc (cargoArgs // { inherit cargoArtifacts; });
        "${pname}-doctest" = pkgs.crane.cargoDocTest (cargoArgs // { inherit cargoArtifacts; });
        "${pname}-nextest" = pkgs.crane.cargoNextest (
          cargoArgs
          // {
            inherit cargoArtifacts;
            cargoExtraArgs = cargoArgs.cargoExtraArgs + " --no-tests pass";
          }
        );
      };

      packages = {
        "${pname}" = pkgs.crane.buildPackage (
          cargoArgs
          // {
            inherit cargoArtifacts;
            doCheck = false;

            postInstall = ''
              mv $out/bin/* $out
              rm -r $out/bin
              cp LICENSE-APACHE $out
              cp LICENSE-MIT $out
            '';
          }
        );
        "${pname}_web" = pkgs.crane.buildTrunkPackage (
          cargoArgs
          // {
            pname = "${pname}_web";
            inherit (pkgs) wasm-bindgen-cli;
          }
        );
      };
    };
}
