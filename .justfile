set shell := ["sh", "-c"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set allow-duplicate-recipes
set positional-arguments
set dotenv-load
set export

alias tmp := tmpdir

bt := '0'

export RUST_BACKTRACE_1 := bt

log := "warn"

export JUST_LOG := (log + "ing" + `grep loop /etc/networks | cut -f2`)

tmpdir  := `mktemp`
version := "0.2.7"
tardir  := tmpdir / "awesomesauce-" + version
foo1    := / "tmp"
foo2_3  := "a/"
tarball := tardir + ".tar.gz"

export RUST_BACKTRACE_2 := "1"
string-with-tab             := "\t"
string-with-newline         := "\n"
string-with-carriage-return := "\r"
string-with-double-quote    := "\""
string-with-slash           := "\\"
string-with-no-newline      := "\
"

# Newlines in variables
single := '
hello
'


alias d := doc
alias l := nix-lint
alias uf := nix-update-flake-dependencies
alias uc := update-cargo-dependencies
#alias r := run
alias t := cargo-test
alias b := build
alias rr := run-release
alias cw := cargo-watch

default:
    @just --choose

tmpdir:
  sh -c  TMPDIR=$(mktemp);export TMPDIR; echo $TMPDIR; touch $TMPDIR/file

clippy:
    cargo clippy --all-targets --all-features

nix-actionlint:
    nix develop .#actionlintShell --command actionlint

deny:
    cargo deny check

cargo-test:
    cargo test

nix-cargo-diet:
    nix develop .#lintShell --command cargo diet

nix-cargo-tarpaulin:
    nix develop .#lintShell --command cargo tarpaulin --out html --exclude-files "benches/*"

nix-cargo-public-api:
    nix develop .#lintShell --command cargo public-api

nix-cargo-diff:
    nix develop .#lintShell --command cargo public-api diff

nix-lint:
    nix develop .#lintShell --command cargo diet
    nix develop .#lintShell --command cargo deny check licenses sources
    nix develop .#lintShell --command typos
    nix develop .#lintShell --command lychee *.md
    nix develop .#fmtShell --command treefmt --fail-on-change
    nix develop .#lintShell --command cargo udeps
    nix develop .#lintShell --command cargo machete
    nix develop .#lintShell --command cargo outdated
    nix develop .#lintShell --command taplo lint
    nix develop .#actionlintShell --command actionlint --ignore SC2002
    cargo check --future-incompat-report
    nix flake check

keygen:
    cargo run --bin keygen rand --json > identity.json

autonat-client:
    cargo run --bin   autonat_client

autonat-server:
    cargo run --bin   autonat_server

autonatv2-client:
    cargo run --bin   autonatv2_client

autonatv2-server:
    cargo run --bin   autonatv2_server

browser-webrtc-example:
    cargo run --bin   browser-webrtc-example

@tabs-example *args='':
    bash -c 'while (( "$#" )); do cargo run --example tabs -- $1; shift; done' -- "$@"

@libp2p-chat-example *args='':
    bash -c 'while (( "$#" )); do cargo run --example libp2p_chat -- $1; shift; done' -- "$@"
@libp2p-tabs-example *args='':
    bash -c 'while (( "$#" )); do cargo run --example libp2p_tabs -- $1; shift; done' -- "$@"

install-autonat_client:
    @just exec "cargo install --bin autonat_client --path examples/autonat"
install-autonat_server:
    @just exec "cargo install --bin autonat_server --path examples/autonat"
install-autonatv2_client:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-autonatv2_server:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-browser-webrtc-example:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-chat-example:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-dcutr-example:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-distributed-commit-list:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-distributed-key-value-store-example:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-file-sharing-example:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-gnostr-p2p:
    @just exec "cargo install --bin gnostr-p2p --path libp2p"
install-hole-punching-tests:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-identify-example:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-ipfs-kad-example:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-ipfs-private-example:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-keygen:
    @just exec "cargo install --bin keygen --path misc/keygen"
install-libp2p-server:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-metrics-example:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-native_ping:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-perf:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-ping-example:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-relay-server-example:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-rendezvous-example:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-rzv-discover:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-rzv-identify:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-rzv-register:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-stream-example:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-upnp-example:
    @just exec "cargo install --bin chat-example --path examples/chat"
install-wasm_ping:
    @just exec "cargo install --bin chat-example --path examples/chat"

dcutr-example:
    cargo run --bin   dcutr-example

distributed-key-value-store-example:
    cargo run --bin   distributed-key-value-store-example

distributed-commit-list:
    cargo run --bin   distributed-commit-list

@exec *args='':
  bash -c 'while (( "$#" )); do $1; shift; done' -- "$@"

#just file-sharing-example provide README.md README.md
file-sharing-example *args='':
  @just exec "cargo b --bin file-sharing-example"
  bash -c 'while (( "$#" )); do cargo run --bin file-sharing-example -- $1 --path $2 --name $3; shift; done' -- "$@"

hole-punching-tests:
    cargo run --bin   hole-punching-tests

identify-example:
    cargo run --bin   identify-example

ipfs-kad-example:
    cargo run --bin   ipfs-kad-example

ipfs-private-example:
    cargo run --bin   ipfs-private-example

libp2p-server:
    cargo run --bin   libp2p-server

metrics-example:
    cargo run --bin   metrics-example

native_ping:
    cargo run --bin   native_ping

perf:
    cargo run --bin   perf

ping-example:
    just relay-server-example & \
    cargo run --bin   ping-example --  /ip4/127.0.0.1/tcp/1111

relay-server-example:
    cargo run --bin   relay-server-example --  --secret-key-seed 0 --port 1111

rendezvous-example:
    cargo run --bin   rendezvous-example

rzv-discover:
    cargo run --bin   rzv-discover

rzv-identify:
    cargo run --bin   rzv-identify

rzv-register:
    cargo run --bin   rzv-register

stream-example:
    cargo run --bin   stream-example

upnp-example:
    cargo run --bin   upnp-example

wasm_ping:
    cargo run --bin   wasm_ping

build:
    cargo build

run-release:
    cargo run --release

doc:
    cargo doc --open --offline

# Update and then commit the `Cargo.lock` file
update-cargo-dependencies:
    cargo update
    git add Cargo.lock
    git commit Cargo.lock -m "update(cargo): \`Cargo.lock\`"

# Future incompatibility report, run regularly
cargo-future:
    cargo check --future-incompat-report

nix-update-flake-dependencies:
    nix flake update --commit-lock-file

cargo-watch:
    cargo watch -x check -x test -x build

# build all examples
nix-examples:
    nix develop --command $SHELL
    example_list=$(cargo build --example 2>&1 | sed '1,2d' | awk '{print $1}')

    # Build each example
    # shellcheck disable=SC2068
    for example in ${example_list[@]}; do
    cargo build --example "$example"
    done

nix-examples-msrv:
    set -x
    nix develop .#msrvShell --command
    rustc --version
    cargo --version
    example_list=$(cargo build --example 2>&1 | grep -v ":")

    # Build each example
    # shellcheck disable=SC2068
    for example in ${example_list[@]}; do
    cargo build --example "$example"
    done
