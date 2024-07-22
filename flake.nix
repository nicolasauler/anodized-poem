{
  description = "Anodized poem devShell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
        # rust = pkgs.rust-bin.beta.latest.default;
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        with pkgs; {
          devShells.default = mkShell {
            buildInputs = [
              bacon
              cargo-expand
              cargo-nextest
              jq
              nixpkgs-fmt
              openssl
              pkg-config
              postgresql
              python3
              rust
              sqlx-cli
            ];

            shellHook = ''
              # export DATABASE_URL=postgres://postgres:postgres@localhost:17144/finnish
              # export DATABASE_URL=postgres://postgres:postgres@localhost:21372/finnish
            '';
          };
        }
    );
}
