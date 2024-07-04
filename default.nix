{ pkgs, lib, ... }:
pkgs.stdenv.mkDerivation rec {
  system = "x86_64-linux";
  name = "theme";
  src = ./src;
  buildInputs = [ pkgs.nushell ];

  phases = [
    "buildPhase"
    "installPhase"
  ];
  buildPhase = ''
    mkdir -p $out/bin
    cp -r ${src}/* $out/bin
    chmod +x $out/bin/main.nu

    ${pkgs.nushell}/bin/nu $out/bin/main.nu
  '';
}
