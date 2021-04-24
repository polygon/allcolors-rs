{ sources ? import ./nix/sources.nix
, pkgs ? import sources.nixpkgs {}
}:
pkgs.stdenv.mkDerivation {
  name = "allcolors-shell";
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libxcb
  ];

  RUST_BACKTRACE="full";
  LD_LIBRARY_PATH = with pkgs.xlibs; "${libX11}/lib:${libXcursor}/lib:${libXrandr}/lib:${libXext}/lib:${pkgs.xorg.libxcb}/lib";
}

