{ sources ? import ./nix/sources.nix
, pkgs ? import sources.nixpkgs {}
, rustPlatform ? pkgs.rustPlatform
, lib ? pkgs.lib
, gi ? import sources."gitignore.nix" {}
}:
let
  buildInputs = with pkgs; [
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    libGL
    mesa
  ];
  inherit (gi) gitignoreSource;
  rpath = lib.makeLibraryPath buildInputs;
in
  rustPlatform.buildRustPackage rec {
    pname = "allcolors-rs";
    version = "0.1.0";

    src = gitignoreSource ./.;
    cargoSha256 = "403c058b029837c8d4de356b13aa13a5301a6e0bcbaaad79601ab0fd14803b07";

    postFixup = "patchelf --set-rpath ${rpath} $out/bin/${pname}";
  }
