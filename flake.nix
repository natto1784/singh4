{
  description = "singh4 discord bot";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixos-unstable;
    utils.url = github:numtide/flake-utils;
    rust-overlay.url = github:oxalica/rust-overlay;
  };

  outputs = { self, nixpkgs, utils, rust-overlay }:
    utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
        rec {
          devShells = with pkgs; {
            default = mkShell
              {
                buildInputs = with gst_all_1; [
                  rust-bin.nightly.latest.default
                  rust-analyzer
                  ffmpeg
                ];
              };
            bare = mkShell
              {
                buildInputs = [
                  rust-bin.nightly.latest.default
                ];
              };
            withLSP = mkShell
              {
                buildInputs = [
                  rust-bin.nightly.latest.default
                  rust-analyzer
                ];
              };
          };
          devShell = devShells.default;
          defaultPackage = pkgs.rustPlatform.buildRustPackage rec {
            pname = "singh4";
            version = "0.1.0";
            src = ./.;
            nativeBuildInputs = with pkgs; [
              rust-bin.nightly.latest.default
            ];
            buildInputs = with pkgs; [
              ffmpeg
            ];
            cargoSha256 = "sha256-0rzz+xOQnZYwygReKobNvUAOMeRrFZj7FUVSkAObm7I=";
          };
        }
      );
}
