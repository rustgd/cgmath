TARGET = lmath

ROOT_DIR = .

SRC_DIR        = $(ROOT_DIR)/src
SRC_CRATE      = $(TARGET).rc
BUILD_DIR      = $(ROOT_DIR)/lib

TEST           = $(TARGET)
TEST_BUILD_DIR = $(ROOT_DIR)/test

.PHONY: test

$(TARGET):
	@echo "Building $(TARGET)..."
	@mkdir -p $(BUILD_DIR)
	@rustc $(SRC_DIR)/$(SRC_CRATE) --out-dir=$(BUILD_DIR)
	@echo "Success"
	
all: $(TARGET)

test:
	@echo "Building unit tests for $(TARGET)..."
	@mkdir -p $(TEST_BUILD_DIR)
	@rustc $(SRC_DIR)/$(SRC_CRATE) --test --out-dir=$(TEST_BUILD_DIR)
	@echo "Success"
	@$(TEST_BUILD_DIR)/$(TARGET)

clean:
	rm -R -f $(BUILD_DIR)
	rm -R -f $(TEST_BUILD_DIR)