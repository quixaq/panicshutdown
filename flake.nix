{
  description = "a minimal utility for an almost instant safe-ish shutdown";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;
        panicshutdown = craneLib.buildPackage {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          strictDeps = true;
        };
      in
      {
        packages.default = panicshutdown;

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
          packages = with pkgs; [
            cargo
            rustc
            rust-analyzer
          ];
        };
      }
    );
}
