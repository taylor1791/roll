{
  description = "CLI for dice rolling";

  outputs = { self, nixpkgs }: {
    # Used by `nix develop`
    devShell = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      pkgs.mkShell {
        buildInputs = [
          pkgs.cargo
          pkgs.clippy
          pkgs.gnuplot # For criterion
          pkgs.rust-analyzer
          pkgs.rustc
          pkgs.rustfmt
        ];
      }
    );
  };
}
