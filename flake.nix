{
    inputs = {
        # a better way of using the latest stable version of nixpkgs
        # without specifying specific release
        nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/*.tar.gz";
    };
    outputs = { self, nixpkgs }: let
        supportedSystems = [
            "aarch64-darwin"
            "aarch64-linux"
            "x86_64-darwin"
            "x86_64-linux"
        ];
        forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
            pkgs = import nixpkgs { inherit system; };
        });
    in {
        packages = forEachSupportedSystem ({ pkgs, ... }: let
            manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        in {
            default = pkgs.rustPlatform.buildRustPackage {
                pname = "melt";
                version = manifest.version;
                src = ./.;
                cargoLock = {
                    lockFile = ./Cargo.lock;
                    outputHashes."wheel-0.15.0" = "sha256-HYut/JuzqonVolT4KzLR41MdGxDmFqmonq+CfOISTvs=";
                };
            };
        });
    };
}
