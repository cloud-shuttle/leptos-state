{
  description = "Leptos State - State management for Leptos applications";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustToolchain
            
            # Node.js and pnpm
            nodejs_20
            nodePackages.pnpm
            
            # Platform-specific dependencies
            pkg-config
            openssl
            curl
            git
            
            # Development tools
            cargo-watch
            wasm-pack
            trunk
          ] ++ (if pkgs.stdenv.isLinux then [
            # Linux-specific Playwright dependencies
            libxkbcommon
            libdrm
            mesa
            xorg.libX11
            xorg.libXcomposite
            xorg.libXcursor
            xorg.libXdamage
            xorg.libXext
            xorg.libXfixes
            xorg.libXi
            xorg.libXrandr
            xorg.libXrender
            xorg.libXScrnSaver
            xorg.libXtst
          ] else []);

          shellHook = ''
            echo "🚀 Leptos State Development Environment"
            echo "📦 Rust: $(rustc --version)"
            echo "📦 Node: $(node --version)"
            echo "📦 pnpm: $(pnpm --version)"
            echo ""
            echo "Available commands:"
            echo "  make test-web     - Test web examples with Playwright"
            echo "  make build-wasm   - Build WASM examples"
            echo "  make serve        - Serve examples for testing"
            echo "  make clean        - Clean build artifacts"
            echo ""
            echo "To install Playwright browsers:"
            echo "  pnpm exec playwright install"
          '';

          # Environment variables
          PLAYWRIGHT_BROWSERS_PATH = "0";
          RUST_BACKTRACE = "1";
        };
      }
    );
}
