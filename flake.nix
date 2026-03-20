{
  description = "";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage (finalAttrs: {
        pname = "slip";
        version = "1.1.0";
        src = ./.;
        cargoHash = "sha256-lA+7hcPJWS34d5M4GOAFUBokzFYzza1QOxqrxlpp63M=";
      });
      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rustc
        ];
      };
    };
}
