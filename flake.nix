{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };
  outputs = { self, nixpkgs, flake-utils, fenix }: flake-utils.lib.eachDefaultSystem (system:
    let pkgs = import nixpkgs { inherit system; };
    in {
      devShells.default = pkgs.mkShell {
        packages = with fenix.packages.${system}; [(combine [
          stable.toolchain
          targets.wasm32-wasi.stable.rust-std
          targets.x86_64-unknown-linux-gnu.stable.rust-std
          targets.x86_64-pc-windows-gnu.stable.rust-std
          targets.x86_64-pc-windows-msvc.stable.rust-std
        ])];
      };
      devShells.nightly = pkgs.mkShell {
        packages = [ fenix.packages.${system}.complete.toolchain ];
      };
      devShells.cross = pkgs.mkShell {
        packages = with pkgs; [ cargo-cross rustup ];
      };
    });
}
