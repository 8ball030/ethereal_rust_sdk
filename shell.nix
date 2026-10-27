let
  pkgs = import <nixpkgs> { };
  openapi-generator-cli-7_13_0 =
    pkgs.openapi-generator-cli.overrideAttrs (old: rec {
      version = "7.13.0";
      src = pkgs.fetchurl {
        url = "https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/${version}/openapi-generator-cli-${version}.jar";
        sha256 = "sha256-0G2kaAm2L96cp6ism9OZv7omUWYbF+JMqlMDQtBoH+I=";
      };
      dontUnpack = true;
      dontBuild = true;
      patches = [];
      installPhase = ''
        runHook preInstall
        install -D $src $out/share/java/openapi-generator-cli.jar
        makeWrapper ${pkgs.jre}/bin/java $out/bin/openapi-generator-cli \
          --add-flags "-jar $out/share/java/openapi-generator-cli.jar"
        runHook postInstall
      '';
    });
in
pkgs.mkShell {
  packages = with pkgs; [
    # rustup alone, so rust-toolchain.toml picks the compiler
    rustup
    # make codegen
    python3
    jq
    redocly
    openapi-generator-cli-7_13_0 # CI pins 7.13.0; nixpkgs' version drifts the generated code
  ];
}
