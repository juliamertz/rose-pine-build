{
  lib,
  stdenv,
  system,
  callPackage,
  autoPatchelfHook,
  glibc,
  libgcc,

  manifest,
  ...
}:
stdenv.mkDerivation {
  inherit (manifest.package) name version;

  src = (callPackage ./generated.nix { }).${system};

  nativeBuildInputs = lib.optionals stdenv.isLinux [
    autoPatchelfHook
  ];

  buildInputs = lib.optionals stdenv.isLinux [
    glibc
    libgcc
  ];

  sourceRoot = ".";

  unpackPhase = ''
    runHook preUnpack
    tar xf $src
    runHook postUnpack
  '';

  installPhase = ''
    runHook preInstall
    install -m755 -D rose-pine-build $out/bin/rose-pine-build
    runHook postInstall
  '';

  meta = with lib; {
    homepage = "https://github.com/juliamertz/rose-pine-build";
    description = "";
    platforms = platforms.all;
    mainProgram = "rose-pine-build";
  };
}
