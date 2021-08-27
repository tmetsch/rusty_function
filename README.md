
# Skeleton for Rust based Azure Functions.

This repository includes all the necessary parts to develop 
[Azure function](https://azure.microsoft.com/en-us/services/functions) using the 
[Rust programming](https://www.rust-lang.org/) language.

Key files to keep in mind when developing your function:

  * [main.rs](src/main.rs) - your logic goes here.
  * [host.json](host.json) - define the executable in this file,
  * [function.json](RustFunction/function.json) - while this file is used to define how Azure while trigger the
    function. Note the *$return* value configured here.

For more information 
[read this blog post](https://www.nohuddleoffense.de/2021/05/14/write-your-functions-in-rust-with-azure-rocket/) 
and the corresponding documentation on Azure 
[here](https://docs.microsoft.com/en-us/azure/azure-functions/create-first-function-vs-code-other?tabs=rust%2Clinux).
