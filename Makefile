.PHONY: clean windows_msvc windows_gnu windows_all
clean:
	cargo clean
windows_msvc:
	cargo xwin build --release --target x86_64-pc-windows-msvc
windows_gnu:
	cargo xwin build --release --target x86_64-pc-windows-gnu
windows_all: windows_msvc windows_gnu
