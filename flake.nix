{
    inputs.flake.url = "github:fenhl/flake";
    outputs = attrs: attrs.flake.lib {
        packages.default = { pkgs, ... }: let
            manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        in pkgs.rustPlatform.buildRustPackage {
            inherit (manifest) version;
            pname = "melt";
            src = ./.;
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
                    --zsh <(
                        COMPLETE=zsh ${melt}
                        # fix completion not working on first Tab press of shell session
                        echo '_clap_dynamic_completer_melt'
                    )
            '';
        };
    };
}
