{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            just
            lld
            cargo-watch
            cargo-nextest
            mosquitto
            openssl_3
            pkg-config
          ];

           shellHook = with pkgs; ''
              export LD_LIBRARY_PATH=${openssl_3.out}/lib:$LD_LIBRARY_PATH
            '';
        };
      });
}
