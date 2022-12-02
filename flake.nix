{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.nixpkgs.follows = "nixpkgs";
    flix-jar = {
        url = "https://github.com/flix/flix/releases/download/v0.34.0/flix.jar";
        flake = false;
    };
  };

  outputs = { self, nixpkgs, flix-jar, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        # Now how I build the flix thing here!?
      in
      {
        devShell = pkgs.mkShell {
            buildInputs = with pkgs; [ jdk  ];
        };
      });
}
