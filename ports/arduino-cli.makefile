ARDUINO_VERSION = 0.2.1-alpha.preview

install:
	ifeq ($(OS),Windows_NT)
    @echo "Not for windows"
	else
	  UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
			UNAME_P := $(shell uname -p)
			ifeq ($(UNAME_P),x86_64)
				@tar xvjf "arduino-cli-$(ARDUINO_VERSION)-linux64.tar.bz2"
				@mv "arduino-cli-$(ARDUINO_VERSION)-linux64" "arduino-cli"
				@rm "arduino-cli-$(ARDUINO_VERSION)-linux64.tar.bz2"
			endif
			ifneq ($(filter %86,$(UNAME_P)),)
				@tar xvjf "arduino-cli-$(ARDUINO_VERSION)-linux32.tar.bz2"
				@mv "arduino-$(ARDUINO_VERSION)-linux32" "arduino-cli"
				@rm "arduino-cli-$(ARDUINO_VERSION)-linux32.tar.bz2"
			endif
			ifneq ($(filter arm%,$(UNAME_P)),)
				@tar xvjf "arduino-cli-$(ARDUINO_VERSION)-linux32.tar.bz2"
				@mv "arduino-cli-$(ARDUINO_VERSION)-linuxarm" "arduino-cli"
				@rm "arduino-cli-$(ARDUINO_VERSION)-linuxarm.tar.bz2"
			endif
    endif
    ifeq ($(UNAME_S),Darwin)
      @unzip "arduino-cli-$(ARDUINO_VERSION)-osx.zip"
			@mv "arduino-cli-$(ARDUINO_VERSION)-osx" "arduino-cli"
			@rm "arduino-cli-$(ARDUINO_VERSION)-osx.zip"
    endif
	endif
