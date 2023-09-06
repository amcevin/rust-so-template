#include <stdio.h>

// 声明 Rust 函数的原型
extern int add(const char* a, const char* b);

int main() {
    char* str1 = "42aa";
    char* str2 = "23";

    // 调用 Rust 函数
    int result = add(str1, str2);

    printf("Result: %d\n", result);

    return 0;
}
