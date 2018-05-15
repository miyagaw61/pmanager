all: target/debug/pmanager
	target/debug/pmanager 2 "echo hoge" "echo fuga"

target/debug/pmanager: src/main.rs
	cargo build
