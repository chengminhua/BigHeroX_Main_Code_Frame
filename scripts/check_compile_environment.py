from base import run, cmd_test_env

CMD_LIST = [
    # Visual Studio
    'devenv.com /Help',
    # Rust Toolchain
    'rustup --version',
    'cargo --version',
    'rustc --version',
    # OpenCV
    'opencv_version',
    cmd_test_env('OPENCV_INCLUDE_PATHS'),
    cmd_test_env('OPENCV_LINK_LIBS'),
    cmd_test_env('OPENCV_LINK_PATHS'),
    # LLVM
    'clang -v',
    'llvm-config --version',
    cmd_test_env('LIBCLANG_PATH'),
    # CMake
    'cmake --version',
    # Test: should failed
    # test_env_cmd('LIBCLANG_PAT'),
]

if __name__ == "__main__":
    print("环境配置检测开始！全程需时30-60s。")
    print("如果出现“命令执行失败”，说明环境配置未完成。")
    for cmd in CMD_LIST:
        run(cmd)
    print("恭喜！环境配置完成！")
