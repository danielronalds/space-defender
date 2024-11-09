run:
    cargo run

# Runs project with nix-shell, with required libraries
nix-run:
    nix-shell -p SDL2 SDL2_ttf SDL2_image --command "cargo run"
