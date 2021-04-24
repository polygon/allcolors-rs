{ sources ? import ./nix/sources.nix
, pkgs ? import sources.nixpkgs {}
, rustPlatform ? pkgs.rustPlatform
, lib ? pkgs.lib
}:
let inherit (import sources."gitignore.nix" {}) gitignoreSource;
in
  rustPlatform.buildRustPackage rec {
    pname = "allcolors-rs";
    version = "0.1.0";

    src = gitignoreSource ./.;
    cargoSha256 = "1jwm7y8klq9znq66q9w56gb1bkk9lk550fa2b03khfhl0nrjf8dw";
  }
