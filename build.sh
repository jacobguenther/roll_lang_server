#!/bin/bash

cd roll_lang_frontend
	bash build.sh
cd ..

cargo build

mkdir www

cp -rv roll_lang_frontend/dist/* www/