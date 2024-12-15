{
  description = "Voxel Cone Tracing implementation in Rust and OpenGL";

  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixos-unstable;
    rust-overlay.url = github:oxalica/rust-overlay;
    utils.url = github:numtide/flake-utils;
  };

  outputs = { self, nixpkgs, rust-overlay, utils, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          (import rust-overlay)
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.nightly."2024-05-28".default; # 1.80
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            mesa
            libGL
            libGLU
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXinerama
            xorg.libXi
            rustToolchain
            cmake
            tokei
          ];

          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.mesa}/lib:${pkgs.libGL}/lib:${pkgs.libGLU}/lib:$LD_LIBRARY_PATH
          '';
        };
      }
    );
}
