{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        buildInputs = with pkgs; [
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          libGL
        ];
        rpath = pkgs.lib.makeLibraryPath buildInputs;
      in rec {
        packages.allcolors-rs = pkgs.rustPlatform.buildRustPackage rec {
          pname = "allcolors-rs";
          version = "0.1.0";

          src = ./.;
          cargoSha256 = "sha256:01rvh0agvc0sc1wsvanb1dp1lc552fm16srmvvachdwq0a5hag20";

          postFixup = "patchelf --set-rpath ${rpath} $out/bin/${pname}";
        };

        defaultPackage = packages.allcolors-rs;
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            rust-analyzer
            clippy
            pkg-config
          ];
          inherit buildInputs;

          LD_LIBRARY_PATH = "${rpath}";
          RUST_BACKTRACE=1;
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
