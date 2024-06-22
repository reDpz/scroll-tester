{
  description = "Rust raylib flake";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShell = pkgs.mkShell
          {
            packages = with pkgs; [
              clang
              llvmPackages_latest.bintools
              rustup
              wayland
              glfw
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
              xorg.libXinerama
            ];

            # Headers
            inputsFrom = with pkgs; [
              libGL
              xorg.libXrandr
              xorg.libXinerama
              xorg.libXcursor
              xorg.libXi
              clang
              llvmPackages_latest.bintools
              rustup
            ];

            # project specific export
            LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
              libGL
              xorg.libXrandr
              xorg.libXinerama
              xorg.libXcursor
              xorg.libXi
            ];

            LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];


            shellHook = ''
              export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
              export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
            '';
          };
      });
}
