npx tailwindcss -i ./input.css -o ./public/tailwind.css
dx build --features web --release
cargo run --features ssr --release
