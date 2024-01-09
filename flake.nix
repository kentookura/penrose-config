{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      with pkgs; {
        packages.default = pkgs.rustPlatform.buildRustPackage {

          pname = "penrose-config";
          version = "0.0.1";
          src = ./.;
          cargoLock = { lockFile = ./Cargo.lock; };
          buildInputs = [ xorg.libX11 xorg.libXft ];
          nativeBuildInputs = with pkgs; [ xorg.xmodmap pkg-config ];
          PKG_CONFIG_PATH = "${pkgs.xorg.libX11}/lib/pkgconfig:${pkgs.xorg.libXft}/lib/pkgconfig:${pkgs.fontconfig}/lib/pkgconfig";
        };
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            xorg.xmodmap
            xorg.libX11
            xorg.libXft
            eza
            fd
            rust-analyzer-unwrapped
            (rust-bin.beta.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer-preview" ];
            })
            # rust-bin.beta.latest.rust-analyzer-preview
          ];

          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
        };
      });
}
