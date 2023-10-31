{ lib, pkgs, system, build_inputs, native_build_inputs, makeRustPlatform }:
let
  wasmTarget = "wasm32-unknown-unknown";

  rustBin = pkgs.rust-bin.stable.latest.default;
  rustBinWasm = rustBin.override { targets = [ wasmTarget ]; };

  rustPlatform = makeRustPlatform {
    cargo = rustBin;
    rustc = rustBin;
  };

  rustPlatformWasm = makeRustPlatform {
    cargo = rustBinWasm;
    rustc = rustBinWasm;
  };

  common = {
    version = "0.0.0";
    src = ./.;
    cargoLock.lockFile = ./Cargo.lock;
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

    buildInputs = build_inputs;
    nativeBuildInputs = native_build_inputs;

    LD_LIBRARY_PATH = lib.makeLibraryPath build_inputs;
  };
in {
  lib = rustPlatform.buildRustPackage (common // { pname = "bevy_vrm"; });
  wasm = rustPlatformWasm.buildRustPackage (common // {
    pname = "bevy_vrm";
    buildPhase = ''
      cargo build --target ${wasmTarget} --profile wasm-release
    '';
    installPhase = ''
      mkdir -p $out/lib
      cp target/${wasmTarget}/wasm-release/*.wasm $out/lib/
    '';
  });
}
