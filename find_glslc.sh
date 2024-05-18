#!/bin/bash
set -e

BIN=""

if [ -n "$PSVersionTable" ]; then
    # Windows
    command_exist() { pwsh -Command "(Get-Command -CommandType Application '$1').Path"; }
else
    command_exist() { command -v "$1"; }
fi

host=$(hostname)
suffix=".42paris.fr"

if [ -e "$(command_exist glslc)" ]; then
	BIN="glslc"
elif [ -e "$(command_exist glslc.exe)" ]; then
    # support WSL
    BIN="$(command_exist glslc.exe)"
elif [[ ! $host == *"$suffix" ]]; then
    >&2 echo "Cannot find glslc binary. Make sure you have VulkanSDK installed and added to PATH."
    exit 1
else
	SOURCE_DIR=$HOME/goinfre/shaderc
	BUILD_DIR=$HOME/sgoinfre/shaderc_build

	if ! [ -d "$BUILD_DIR" ]; then
		git clone https://github.com/google/shaderc "$SOURCE_DIR"
		cd "$SOURCE_DIR"
		./utils/git-sync-deps

		mkdir -p "$BUILD_DIR"
		cd "$BUILD_DIR"
		cmake -GNinja -DCMAKE_BUILD_TYPE=Release "$SOURCE_DIR"
		ninja
		# ctest # optional
	fi
	BIN=$BUILD_DIR/glslc/glslc
fi

echo "$BIN"
