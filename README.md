# clowt
A sample RAG implementation in Rust based on the watsonx example.

https://dataplatform.cloud.ibm.com/exchange/public/entry/view/ebeb9fc0-9844-4838-aff8-1fa1997d0c13?context=wx&audience=wdp


## notes

1. Services used in this example can be provisioned using OpenTofu and the definitions in the [infra](./infra) of this project

1. Only tested on Linux


1. Install libtorch 
   Download https://download.pytorch.org/libtorch/cu118/libtorch-cxx11-abi-shared-with-deps-2.1.0%2Bcu118.zip and set up exports

   ```
   export LIBTORCH=/DOWNLOAD_PATH/libtorch-cxx11-abi-shared-with-deps-2.1.0+cu118/libtorch
   export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
   ```
   It's recommmended to add these to your global environment variables so that rust-analyzer doesn't produce warnings.

