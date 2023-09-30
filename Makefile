INSTALL_PATH ?= /usr/local/bin
ALIAS_RS_BINARY = ./target/release/alias-rs

all:
	cargo build --release

install: $(ALIAS_RS_BINARY)
	mkdir -p $(INSTALL_PATH)
	cp -f $(ALIAS_RS_BINARY) $(INSTALL_PATH)
	chmod 755 $(INSTALL_PATH)/alias-rs

uninstall:
	rm -f $(INSTALL_PATH)/alias-rs

clean:
	cargo clean

.PHONY: all install uninstall clean
