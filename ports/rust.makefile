EXECUTABLES = sh curl
K := $(foreach exec,$(EXECUTABLES),\$(if $(shell which $(exec)),$(info $(exec) OK),$(error $(exec) not found in PATH)))
N := $(if $(shell which rustup),$(error rust installed),not installed)
install:
	@curl https://sh.rustup.rs -sSf | sh
