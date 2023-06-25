clear
cargo fmt
cross build --bin adept_shop_titans_simulator --release --target x86_64-pc-windows-gnu
rm -rf releases/build
mkdir -p releases/build/adept/adept_data
cp -r target/x86_64-pc-windows-gnu/release/adept_shop_titans_simulator.exe releases/build/adept/adept_shop_titans_simulator.exe
cp -r bundle releases/build/adept/adept_data/bundle
cp -r config releases/build/adept/adept_data/config
mkdir -p releases/0.1.1
cd releases/build
zip -r ../../releases/0.1.1/adept_0.1.1_windows.zip adept
cd ../..