#!/bin/sh

PREFIX=${@:-/usr/local}
DIR=$(cd $(dirname $0) && pwd)

mkdir -p "$PREFIX/bin"
cp "$DIR/project_root" "$PREFIX/bin"

mkdir -p "$PREFIX/share/man/man1"
cp "$DIR/project_root.1" "$PREFIX/share/man/man1"

if [ command -v mandb >/dev/null 2>&1 ]; then
  mandb
fi
