TARGET			:= target/debug/scop

SHADERS_DIR		:= shaders
SHADERS_OUT_DIR	:= shaders_compiled

SHADER_COMPILER	:= $(shell ./find_glslc.sh)
ifeq "$(SHADER_COMPILER)" ""
	break;
endif

define outpath
$(addsuffix .$(1), $(subst $(SHADERS_DIR),$(SHADERS_OUT_DIR),$(2)))
endef

SHADER_FILES	:= $(filter-out /header/,$(wildcard $(SHADERS_DIR)/*.vert $(SHADERS_DIR)/*.frag))
SHADER_OUTPUTS	:= $(call outpath,spv,$(SHADER_FILES)) $(call outpath,spvasm,$(SHADER_FILES))

all: build $(SHADER_OUTPUTS)

build:
	cargo build $(cargo_flags)

shader: $(SHADER_OUTPUTS)

%.spv:
	mkdir -p $(dir $@)
	$(SHADER_COMPILER) -MD -c -g -O -o $@ $<

%.spvasm:
	mkdir -p $(dir $@)
	$(SHADER_COMPILER) -MD -S -g -O -o $@ $<

define shader_rules
$(call outpath,spv,$1) : $1
$(call outpath,spvasm,$1) : $1
endef

$(foreach shader,$(SHADER_FILES),$(eval $(call shader_rules,$(shader))))

release: cargo_flags += --release
release: all

run:
	RUST_LOG=debug cargo run

clean_shaders:
	$(RM) $(SHADERS_OUT_DIR)/**/*.spv $(SHADERS_OUT_DIR)/**/*.spvdis $(SHADERS_OUT_DIR)/**/*.d

clean: clean_shaders
	cargo clean

re: clean all

.PHONY: all build release run shader clean clean_shaders re
