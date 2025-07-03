#include <stdio.h>
#include "rust_so_example.h"

int main()
{
    char *str1 = "42aa";
    char *str2 = "23";

    // 调用 Rust 函数
    int result = add(str1, str2);

    printf("Result: %d\n", result);

    return 0;
}
