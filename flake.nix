{
  description = "rust raylib flake";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShell = pkgs.mkShell {
          # native packages
          packages = with pkgs; [
            rustup
            cargo-cross
            cmake
            clang
            pkg-config
            wayland
            glfw
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            xorg.libXinerama
            xorg.libXtst
          ];
          # Headers
          inputsFrom = with pkgs; [
            libGL
            xorg.libXrandr
            xorg.libXinerama
            xorg.libXcursor
            xorg.libXi
            xorg.libXtst
          ];

          # project specific export
          LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
            libGL
            xorg.libXrandr
            xorg.libXinerama
            xorg.libXcursor
            xorg.libXi
          ];
          LIBCLANG_PATH = "${pkgs.llvmPackages_latest.libclang.lib}/lib";
        };
      });
}
