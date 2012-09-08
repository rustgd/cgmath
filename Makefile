TARGET = omath

ROOT_DIR = .

SRC_DIR        = $(ROOT_DIR)/src
SRC_CRATE      = $(TARGET).rc
BUILD_DIR      = $(ROOT_DIR)/lib

TEST           = test_$(TARGET)
TEST_DIR 	   = $(ROOT_DIR)/test
TEST_BUILD_DIR = $(TEST_DIR)/build
TEST_CRATE     = $(TEST).rc

$(TARGET):
	@echo "Building $(TARGET)"
	@rustc $(SRC_DIR)/$(SRC_CRATE) --lib -g --out-dir=$(BUILD_DIR)
	@echo "Success! \o/"

all: $(TARGET)

test: all
	@echo "..."
	@echo "Building $(TEST)"
	@rustc --test -L lib $(TEST_DIR)/$(TEST_CRATE) -g --out-dir=$(TEST_BUILD_DIR)
	@echo "Success! \o/"
	@$(TEST_BUILD_DIR)/$(TEST)

clean:
	rm -R -f $(BUILD_DIR)/*
	rm -R -f $(TEST_BUILD_DIR)/*