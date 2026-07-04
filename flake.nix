{
    inputs.nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/*.tar.gz";
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
                cargoLock = {
                    allowBuiltinFetchGit = true; # allows omitting cargoLock.outputHashes
                    lockFile = ./Cargo.lock;
                };
                nativeBuildInputs = with pkgs; [
                    installShellFiles # required for `installShellCompletion` in postInstall hook
                ];
                postInstall = let
                    melt = "${pkgs.stdenv.hostPlatform.emulator pkgs.buildPackages} $out/bin/melt";
                in pkgs.lib.optionalString (pkgs.stdenv.hostPlatform.emulatorAvailable pkgs.buildPackages) ''
                    installShellCompletion --cmd melt \
                        --bash <(COMPLETE=bash ${melt}) \
                        --fish <(COMPLETE=fish ${melt}) \
                        --zsh <(COMPLETE=zsh ${melt})
                '';
                src = ./.;
            };
        });
    };
}
