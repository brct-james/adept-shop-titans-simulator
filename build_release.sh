clear
cargo fmt
cross build --bin adept_shop_titans_simulator --release --target x86_64-pc-windows-gnu
rm -rf release
mkdir release
cp -r target/x86_64-pc-windows-gnu/release/adept_shop_titans_simulator.exe release/adept_shop_titans_simulator.exe
cp -r bundle release/bundle
cp -r config release/config