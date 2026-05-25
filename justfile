set quiet

run FILE:
    cargo run -- run -m {{FILE}}

spec:
    just run ochre-interp/spec.ocr
