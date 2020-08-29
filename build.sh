#!/bin/bash

cd roll_lang_frontend
	bash build.sh
cd ..

cargo build
cp -rv roll_lang_frontend/dist/* www/