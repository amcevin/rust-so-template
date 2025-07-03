# rust so example

rust 编译为跨语言调用的so 模块

## osx change so path

```shell
export DYLD_LIBRARY_PATH=/so/path/:$DYLD_LIBRARY_PATH
```

## linux appaend so path

```shell
export LD_LIBRARY_PATH=/so/path:$LD_LIBRARY_PATH
```

## TODO production 

1. Add Code to src/lib.rs
2. make header
3. add caller from other client such as go-client 
4. make go-client 

## How to run produciton

1. copy the compiled go binary
2. copy the compiled so file
3. update `DYLD_LIBRARY_PATH` or `LD_LIBRARY_PATH`

## how to upgrade so version

1. upload so to the path
2. use ln change the soft link

    ```shell
    ln -sfn librust_so_example.so.1.1.0 librust_so_example.so
    ```

3. restart the binary process if needed

