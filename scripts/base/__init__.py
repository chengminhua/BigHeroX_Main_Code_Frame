import subprocess

def run(
    command: str,
    show_cmd=True,
    hide=False,
    cwd=None,
    exit_on_fail=True,
    prefix="powershell -c "
) -> int:
    if show_cmd:
        print("命令:", command)
        print("执行目录:", cwd)
    stdout = None
    stderr = None
    if hide:
        stdout = subprocess.PIPE
        stderr = subprocess.PIPE
    returncode = subprocess.run(prefix + command, shell=True, stdout=stdout, stderr=stderr, cwd=cwd).returncode
    if exit_on_fail and returncode != 0:
        print("命令执行失败:", command)
        print("退出程序")
        exit(1)
    return returncode;

def cmd_test_env(env: str) -> str:
    return f'Copy-Item -Path Env:\\{env} -Destination Env:\\TEST_ENV -PassThru'

def yes_or_no(input: str) -> bool:
    return input.upper().startswith('Y')

def input_yes_or_no(tips="", default=True) -> bool:
    opt = input(tips + ("[Y/n]:" if default else "[y/N]:"))
    if len(opt) == 0:
        return default
    return yes_or_no(opt)
