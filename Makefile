# Copyright 2013 The Lmath Developers. For a full listing of the authors,
# refer to the AUTHORS file at the top-level directory of this distribution.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

TARGET = lmath

ROOT_DIR = .

SRC_DIR        = $(ROOT_DIR)/src
SRC_CRATE      = $(TARGET).rs
EXTERN_DIR     = $(ROOT_DIR)/extern
BUILD_DIR      = $(ROOT_DIR)/lib

CFG            = --cfg=color --cfg=geom --cfg=noise --cfg=world

TEST           = $(TARGET)
TEST_BUILD_DIR = $(ROOT_DIR)/test

.PHONY: test

$(TARGET):
	@echo "Building $(TARGET)..."
	@mkdir -p $(BUILD_DIR)
	@rustc $(CFG) $(SRC_DIR)/$(SRC_CRATE) --out-dir=$(BUILD_DIR)
	@echo "Success"

all: $(TARGET)

test:
	@echo "Building unit tests for $(TARGET)..."
	@mkdir -p $(TEST_BUILD_DIR)
	@rustc  $(CFG) $(SRC_DIR)/$(SRC_CRATE) --test --out-dir=$(TEST_BUILD_DIR)
	@echo "Success"
	@$(TEST_BUILD_DIR)/$(TARGET)

clean:
	rm -R -f $(BUILD_DIR)
	rm -R -f $(TEST_BUILD_DIR)