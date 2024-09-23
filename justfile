build FLAG="12":
    cargo build -j{{ FLAG }}

arm FLAG="12":
    cargo build -j{{ FLAG }} --target ./aarch64-ineptos.json
