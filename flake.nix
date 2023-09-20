{
  description = "Server app for SlimeVR ecosystem";

  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustTarget =
          pkgs.rust-bin.stable.latest.default.override
          {
            extensions = ["rust-analyzer" "rust-src"];
          };
        nativeBuildInputs = with pkgs; [
          curl
          gcc
          pkg-config
          which
          zlib
        ];
        buildInputs = with pkgs; [
        ];
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs =
            nativeBuildInputs
            ++ [
            ];
          buildInputs =
            buildInputs
            ++ [
              rustTarget
            ];
        };
      }
    );
}
