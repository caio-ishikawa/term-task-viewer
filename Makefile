# Makefile for building, installing, and adding Rust project to PATH

# Configuration

INSTALL_DIR := /usr/local/bin

# Commands
CARGO := cargo
INSTALL := install -m 755
LN := ln -sf

all: install

build:
	$(CARGO) build --release

install: build
	@echo "Installing ttv to $(INSTALL_DIR)"
	@$(INSTALL) target/release/ttv $(INSTALL_DIR)
	@echo "Creating a symbolic link to ttv in $(INSTALL_DIR)"
	@$(LN) $(abspath target/release/ttv) $(INSTALL_DIR)/ttv

uninstall:
	@echo "Removing ttv from $(INSTALL_DIR)"
	@rm -f $(INSTALL_DIR)/ttv

.PHONY: all build install uninstall 

