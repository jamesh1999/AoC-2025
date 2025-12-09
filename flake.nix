{
    description = "Tutorials from the Rust by Example pages";

    inputs.nixpkgs.url = "nixpkgs";

    outputs = {nixpkgs, ...}:
        let
            system = "x86_64-linux";
            pkgs = import nixpkgs { inherit system; };
        in {
            devShells.${system}.default = pkgs.mkShell {
                name = "qmk-default";
                RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
                packages = with pkgs; [
                    rustc
                    cargo
                    clippy
                ];
            };
        };
}