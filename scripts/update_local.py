import os

from base import input_yes_or_no, run
from repo import REPOSITORIES

# 获取当前正在执行的Python脚本的绝对路径
current_file_path = os.path.abspath(__file__)
 
# 获取该脚本所在的目录
current_directory = os.path.dirname(current_file_path)
base_directory = f"{current_directory}/../.."

DEFAULT_UPDATE_SOURCE_DIRS = [
    "D:/BigHeroX RoboCup", 
    "E:/BigHeroX RoboCup", 
    "F:/BigHeroX RoboCup", 
]
UPDATE_SOURCE_CONFIG_FILE = f"{current_directory}/update_local_path"

if __name__ == "__main__":
    print("同步脚本：从本地同步")
    print("作用：为当前仓库指定远端仓库为本地另一仓库（同步源），将其更新同步至当前仓库。")
    print("如当前目录无对应仓库仓库则克隆，已经克隆了就设置远端并拉取。")
    print("相关命令：git clone; git remote; git pull; git status")
    # 获取文件中的更新地址
    update_source_dir_selections_unchecked: list[str] = DEFAULT_UPDATE_SOURCE_DIRS
    try:
        with open(UPDATE_SOURCE_CONFIG_FILE, "rb") as file:
            file_bytes = file.read()
            file_str = file_bytes.decode("utf-8")
            file_lines = file_str.splitlines()
            for i, _ in enumerate(file_lines):
                file_lines[i] = file_lines[i].strip().removesuffix("/").removesuffix("\\")
            print("由文件添加的路径：")
            for dir in file_lines:
                print(dir)
            update_source_dir_selections_unchecked.extend(file_lines)
    except FileNotFoundError as _:
        with open(UPDATE_SOURCE_CONFIG_FILE, "wb") as file:
            file.write(" ".encode("utf-8"))
    # 检查更新地址
    update_source_dir_selections_unchecked.sort()
    update_source_dir_selections = []
    for dir in update_source_dir_selections_unchecked:
        if not os.path.exists(f"{dir}/bigherox-robocup/scripts/update_local.py"):
            continue
        update_source_dir_selections.append(dir)
    # 选择更新地址
    print("请从以下更新地址中选择一个，输入选项前的编号。")
    print("如果没有想要的选项，请修改同目录下新生成的update_local_path文件。")
    for i, dir in enumerate(update_source_dir_selections):
        print(i, dir)
    selection = input("请选择（默认：0）：").strip()
    selected_index = 0
    # 进行更新
    if len(selection) > 0 and (not selection.isdigit()):
        print("应输入数字标号")
        exit()
    if len(selection) > 0 and int(selection) >= len(update_source_dir_selections):
        print("无效选择")
        exit()
    if len(selection) > 0:
        selected_index = int(selection)
    update_source_dir = update_source_dir_selections[selected_index]
    for repository in REPOSITORIES:
        repository_dir = f"{base_directory}/{repository.folder_name}/"
        if os.path.exists(repository_dir):
            print(f"更新仓库：{repository.git_remote_addr()}")
            run(f"git remote add local \"{update_source_dir}/{repository.folder_name}\"", cwd=repository_dir, prefix="")
            run(f"git pull local {repository.default_branch}:{repository.default_branch}", cwd=repository_dir)
            run("git remote remove local", cwd=repository_dir, hide=True)
            print(f"检查仓库状态：{repository.git_remote_addr()}")
            run('git status', cwd=repository_dir)

        else:
            print(f"克隆仓库：{repository.git_remote_addr()}")
            run(f"git clone \"{update_source_dir}/{repository.folder_name}\"", cwd=base_directory, prefix="")

