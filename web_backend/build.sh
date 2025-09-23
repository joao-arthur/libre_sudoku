rm -Rf ./sudoku_engine
rm -Rf ../sudoku_engine
wasm-pack build --target web --out-dir sudoku_engine
mv ./sudoku_engine ../sudoku_engine
