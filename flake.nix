{
  description = "Vicardi - Rust jCard parser";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    flake-utils,
    nixpkgs,
    naersk,
  }:
      flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (nixpkgs) lib;
        naersk = naersk.lib.${system};
      in {
        formatter = pkgs.alejandra;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            clippy
            rustfmt
          ];
        };
      }
    );
}
