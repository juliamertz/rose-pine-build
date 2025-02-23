{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      forEachSystem =
        f:
        nixpkgs.lib.genAttrs (nixpkgs.lib.systems.flakeExposed) (
          system: f nixpkgs.legacyPackages.${system}
        );
    in
    {
      packages = forEachSystem (
        pkgs: with pkgs;
        let manifest = lib.importTOML ./build/Cargo.toml; in
        {
          default = rustPlatform.buildRustPackage {
            inherit (manifest.package) name version;
            buildInputs = [ openssl ];
            nativeBuildInputs = [ pkg-config ];

            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
              allowBuiltinFetchGit = true;
            };
            meta.mainProgram = "rose-pine-build";
          };
        }
      );

      devShells.default = forEachSystem (
        pkgs:
        with pkgs;
        mkShell {
          nativeBuildInputs = [ pkg-config ];
          buildInputs = [
            openssl
            clippy
            rustfmt
            rust-analyzer
          ];
        }
      );
    };
}
