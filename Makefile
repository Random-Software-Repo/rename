
CXX = cargo build --release --target-dir target

PROGRAM = rename

default:
	@echo "Compiles and installs $(PROGRAM)."
	@echo "To compile / build run: "
	@echo "     \"make build\"" 
	@echo "To Insall run:"
	@echo "     \"make install\""
	@echo ""


build:
	@if [ $$USER = root ];\
	then \
		echo "Do not run make to build $(PROGRAM) as root.\nInstalling with make as root is ok.";\
	else \
		$(CXX);\
	fi

clean: 
	rm -rf target

install:
	cp target/release/$(PROGRAM) /usr/local/bin
	chmod 755 /usr/local/bin/$(PROGRAM)
