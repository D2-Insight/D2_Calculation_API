# D2_Calculation_API


Credit goes first and foremost as this would not have been possible without the great community of Destiny Massive Breakdowns and the people in it.
  * Mossy, a majority of damage values are from him and the underlying pve formulas for PL
  * RokDc(from reddit😩) for help with questions and more damage numbers
  * The destiny 2 compendium creators and maintainers for making my life easier
  * Kat the creater of d2foundry.gg for assisting me with JS bug testing and helping me coping with the language


  # Usage

Python:
  * A `.pyi` is included which contains the methods and class available in the py module
  * Unlike javascript, pybindings use different sub modules to sort functions better

JavaScript:
  * fml

this project is not meant to be "run" and is still in beta so i not have fully written up compile instructions. If you want a wasm binary with JavaScript bindings install wasm-pack and run with '--features wasm'. If you desire python run a cargo build with '--features python' and take the .dll compiled and rename it to a .pyd for windows or a .so for mac I believe. Look at pyo3 documentation if you need more help with python. 
