TARGET = wasm32-unknown-unknown
BUILD = cargo build --target $(TARGET)

.PHONY: build
build:
	$(BUILD)
	mv -f target/$(TARGET)/debug/baruwa.wasm ./www/app/baruwa.wasm
	cp -f js/*.js ./www/app/

.PHONY: build_release
build_release:
	$(BUILD) --release
	mv -f target/$(TARGET)/release/baruwa.wasm ./www/app/baruwa.wasm
	cp -f js/*.js ./www/app/

