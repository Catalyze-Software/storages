echo "Starting build..."

echo "Generating candids"
cargo test candid --workspace --exclude common --exclude tests --exclude migration

echo "Building the canisters..."
cargo build --workspace --release --target wasm32-unknown-unknown --exclude common --exclude tests

mkdir -p wasm

echo "Compressing wasm files..."
gzip -r `find target/wasm32-unknown-unknown/release -type f -name "*.wasm"` -f

echo "Coping gzips..."
find target/wasm32-unknown-unknown/release -name \*.wasm.gz -exec cp {} wasm \;

echo "Build complete"
