TAR_NAME="particle-mac.tar.gz"

cargo build --release
(cd target/release && tar -czf $TAR_NAME particle && shasum -a 256 $TAR_NAME)
