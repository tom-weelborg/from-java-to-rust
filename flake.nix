{
  description = "from-java-to-rust";

  inputs = {
    nixpkgs.url = "nixpkgs";
  };

  outputs = inputs:
  let
    forAllSystems = f: builtins.mapAttrs f inputs.nixpkgs.legacyPackages;
  in
  {
    packages = forAllSystems (system: pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          # NodeJS
          nodejs_24
          pnpm_12

          # C
          gcc

          # Java
          jdk25
          maven3

          # Rust
          rustc
          cargo
        ];
      };
    });
  };
}
