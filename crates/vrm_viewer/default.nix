_: {
  perSystem =
    { pkgs, lib, ... }:
    let
      pname = "vrm_viewer";

      buildInputs = pkgs.lib.optionals pkgs.stdenv.isLinux (
        with pkgs;
        [
          alsa-lib
          libX11
          libXcursor
          libXi
          libXrandr
          libxkbcommon
          openssl
          udev
          vulkan-loader
          wayland
        ]
      );

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
        inherit pname;
        inherit src;

        nativeBuildInputs = with pkgs; [ pkg-config ];

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
              cp LICENSE-* $out
            '';
          }
        );
        "${pname}_web" = pkgs.crane.buildTrunkPackage (
          cargoArgs
          // {
            pname = "${pname}_web";
            wasm-bindgen-cli = pkgs.wasm-bindgen-cli_0_2_114;
          }
        );
      };
    };
}
