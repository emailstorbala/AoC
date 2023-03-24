with import <nixpkgs> {};
# https://blog.galowicz.de/2019/04/17/tutorial_nix_cpp_setup/

stdenv.mkDerivation {
  name = "aoc2022";
  src = ./.;

  buildInputs = [ cmake boost fmt ];

  buildPhase = "cmake . && make";

  installPhase = ''
    mkdir -p $out/bin $out/lib
    cp bin/aoc2022 $out/bin/
    cp lib/libUtilities.so $out/lib/
  '';
}
