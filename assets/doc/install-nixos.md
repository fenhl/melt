# Installing `melt` on NixOS

1. Add the following input to your system flake:
    ```nix
    melt = {
        url = "github:fenhl/melt";
        inputs.nixpkgs.follows = "nixpkgs"; # optional, to deduplicate dependencies, omit if your flake doesn't have a nixpkgs input
    };
    ```
2. You can now use the package as `melt.packages.${pkgs.system}.default`. For example, to make it available globally:
    ```nix
    environment.systemPackages = [
        melt.packages.${pkgs.system}.default
    ];
    ```
3. `sudo nixos-rebuild switch`
