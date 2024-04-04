#!/bin/bash
set -e

BIN=""

host=$(hostname -f)
suffix=".42paris.fr"

if [ -e "$(which glslc)" ]; then
	BIN="glslc"
elif [ -e "$(which glslc.exe)" ]; then
    # support WSL
    BIN="$(which glslc.exe)"
elif [[ ! $host == *"$suffix" ]]; then
    >&2 echo "Cannot find glslc binary. Make sure you have VulkanSDK installed and added to PATH."
    exit 1
fi

SOURCE_DIR=$HOME/goinfre/shaderc
BUILD_DIR=$HOME/goinfre/shaderc_build

if ! [ -d $BUILD_DIR ]; then
	git clone https://github.com/google/shaderc $SOURCE_DIR
	cd $SOURCE_DIR
	./utils/git-sync-deps

	mkdir -p $BUILD_DIR
	cd $BUILD_DIR
	cmake -GNinja -DCMAKE_BUILD_TYPE=Release $SOURCE_DIR
	ninja
	# ctest # optional
fi
BIN=$BUILD_DIR/glslc/glslc

echo "$BIN"
