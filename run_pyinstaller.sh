#!/usr/bin/env bash

rm -rf backup.spec build dist

pyinstaller -w -i icons/logo.ico \
  backup.py

mkdir -p dist/backup/icons
cp icons/* dist/backup/icons
