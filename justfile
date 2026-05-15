set quiet

test:
    cargo test -- --nocapture

run FILE:
    cargo run -- eval -I {{FILE}}
