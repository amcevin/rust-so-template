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
