{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
  }:
    utils.lib.eachDefaultSystem (system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in rec
      {
        packages.pkg-warner = naersk-lib.buildPackage {
          pname = "pkg-warner";
          root = ./.;

          PKG_WARNER_PACKAGES = "test1,test2,test3";
          PKG_WARNER_DISTRO = "Test Linux";
          PKG_WARNER_PMAN = "test4";
        };

        packages.default = packages.pkg-warner;

        apps.pkg-warner = utils.lib.mkApp {
          drv = packages.pkg-warner;
        };

        apps.default = apps.pkg-warner;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            cargo-audit
            rustfmt
            clippy

            just
          ];
        };

        formatter = pkgs.alejandra;
      });
}
