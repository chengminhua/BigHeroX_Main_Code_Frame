import os

from base import input_yes_or_no, run
from repo import REPOSITORIES

# 获取当前正在执行的Python脚本的绝对路径
current_file_path = os.path.abspath(__file__)
 
# 获取该脚本所在的目录
current_directory = os.path.dirname(current_file_path)
base_directory = f"{current_directory}/../.."

if __name__ == "__main__":
    print("同步脚本：从远程同步")
    print("作用：从远程更新BigHeroX RoboCup的所有仓库。还没克隆就克隆，已经克隆了就拉取并检查状态。")
    print("相关命令：git clone; git pull; git status")
    update_exists_repo = input_yes_or_no(tips="是否运行脚本？", default=True)
    if not update_exists_repo:
        exit()
    for repository in REPOSITORIES:
        repository_dir = f"{base_directory}/{repository.folder_name}/"
        if os.path.exists(repository_dir):
            print(f"更新仓库：{repository.git_remote_addr()}")
            run("git pull", cwd=repository_dir)
            print(f"检查仓库状态：{repository.git_remote_addr()}")
            run('git status', cwd=repository_dir)

        else:
            print(f"克隆仓库：{repository.git_remote_addr()}")
            run(f"git clone {repository.git_remote_addr()}", cwd=f"{base_directory}")
