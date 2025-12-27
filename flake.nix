{
    description = "Advent of Code 2025";

    inputs.nixpkgs.url = "nixpkgs";

    outputs = {nixpkgs, ...}:
        let
            system = "x86_64-linux";
            pkgs = import nixpkgs { inherit system; };
        in {
            devShells.${system}.default = pkgs.mkShell {
                name = "rust-default";
                RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
                packages = with pkgs; [
                    rustc
                    cargo
                    clippy
                ];
            };
        };
}
