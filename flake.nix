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
    devShells = forAllSystems (system: pkgs: {
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

    packages = forAllSystems (system: pkgs: {
      web-app = {
        java = pkgs.maven3.buildMavenPackage {
          pname = "blog";
          version = "0.1.0";

          src = ./snippets/web-app/java;
          mvnHash = "sha256-P7eufIa8kkmn3WWDUK0vgyf1krRb0rX/B5fmfMEkQMU=";

          mvnJdk = pkgs.jdk25;

          nativeBuildInputs = with pkgs; [
            makeWrapper
          ];

          installPhase = ''
            runHook preInstall

            mkdir -p $out/share/blog

            install -Dm644 target/blog-0.1.0.jar \
              $out/share/blog/blog.jar

            makeWrapper ${pkgs.jdk25}/bin/java $out/bin/blog \
              --add-flags "-jar $out/share/blog/blog.jar"

            runHook postInstall
          '';

          meta = {
            description = "Example Java blog web app with Spring Boot";
          };
        };

        rust = pkgs.rustPlatform.buildRustPackage {
          pname = "blog";
          version = "0.1.0";

          src = ./snippets/web-app/rust;
          cargoLock.lockFile = ./snippets/web-app/rust/Cargo.lock;

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          meta = {
            description = "Example Rust blog web app with Axum";
          };
        };
      };
    });
  };
}
