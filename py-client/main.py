import logging
import ctypes

# 加载 Rust 库
rust_lib = ctypes.CDLL('../target/release/librust_so_example.dylib')
rust_lib.get_pkg_version.restype = ctypes.c_char_p
rust_lib.get_pkg_name.restype = ctypes.c_char_p

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def main():
    logger.info("Starting the application")
    result = rust_lib.add(b"42aa", b"235")
    logger.info(f"Result: {result}")
    
    so_name = rust_lib.get_pkg_name()
    logger.info(f"SO Name: {so_name}")
    
    so_version = rust_lib.get_pkg_version()
    logger.info(f"SO Version: {ctypes.cast(so_version, ctypes.c_char_p).value.decode('utf-8')}")

if __name__ == "__main__":
    main()