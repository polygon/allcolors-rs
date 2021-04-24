{ sources ? import ./nix/sources.nix
, pkgs ? import sources.nixpkgs {}
}:
pkgs.stdenv.mkDerivation {
  name = "allcolors-shell";
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
  ];

  RUST_BACKTRACE = 1;
}

