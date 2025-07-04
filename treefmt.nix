_: {
  projectRootFile = "flake.nix";
  programs = {
    actionlint.enable = true;
    deadnix.enable = true;
    mdformat.enable = true;
    nixfmt.enable = true;
    rustfmt = {
      enable = true;
      edition = "2024";
    };
    statix.enable = true;
    taplo.enable = true;
    yamlfmt.enable = true;
  };
}
