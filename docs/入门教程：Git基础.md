# 入门教程：Git介绍

## Git使用过程中，用到的几个区域

### 1. 存储库（远程）
也就是我们部署在服务器上的代码仓库。

### 2. 存储库（本地）
通常由远程存储库克隆而来。

当然，也可以先在本地建立存储库，将修改提交至本地存储库后，在为本地存储库添加远端目标，将本地存储库的修改同步至指定远端仓库。此操作常用于新建和转移项目。

### 3. 暂存区

### 4. 本地文件

## 常用操作
只是一些常用操作，帮助入门，不包含分支、checkout和Tag等进阶内容。

### 1. 克隆仓库
- 操作位置：远程存储库
- 目标位置：本地存储库、本地文件

以克隆此仓库为例：
```powershell
git clone http://115.157.210.249:8080/bigherox-robocup/docs.git docs
```
（后面的docs表示克隆后新目录的名称）

克隆操作包含：
1. 将远程存储库复制一份到当前目录的指定文件夹下，作为本地存储库。
2. 更新本地文件至本地存储库的最新状态。

### 2. 添加修改
- 操作位置：本地文件
- 目标位置：暂存区

对于当前目录的存储库，添加当前目录（包括子目录）下的所有修改，包括文件添加、修改和删除。
```powershell
git add .
```
如果只想添加某个文件，将`.`改为对应文件路径即可。

### 3. 提交
- 操作位置：暂存区
- 目标位置：本地存储库

```powershell
git commit --message="提交信息"
```
将暂存区的修改提交至本地存储库，并清空暂存区。

### 4. 推送
- 操作位置：本地存储库
- 目标位置：远程存储库

```powershell
git push
```
将本地存储库的提交同步至远程存储库。

### 5. 回退提交
- 操作位置：本地存储库
- 目标位置：本地存储库

回退一步提交：
```powershell
git reset HEAD^
```
此操作会回退一次本地存储库的提交，但保留本地文件的修改（也就是不会动本地文件）。

### 6. 回退本地修改
- 操作位置：本地存储库
- 操作位置：本地文件
```powershell
git restore .
```
将当前目录下的所有文件，回退至本地存储库的最新提交（也就是说在此之后的修改都没了）。

如果只想回退某个文件，将`.`改为对应文件路径即可。

## 入门教程：Git使用Q&A

### Q&A 一、我想把仓库的代码提取出来，修改代码，然后将修改同步到仓库，怎么做？

#### 第一步：同步仓库内容到本地
首先，保证当前网络环境能够访问远端仓库：http://115.157.210.249:8080/

##### 自动方式
```powershell
python ./update_all.py
```
这个脚本的作用是，对于每一个项目仓库：
- 如果仓库未被克隆至本地，则在父目录下克隆该仓库。
- 如果仓库已被克隆至本地，则在对应仓库目录下执行`git pull`。

##### 手动方式
在每个仓库目录下，执行以下命令：
```powershell
git pull
```

##### 故障排除：之前的修改没同步上去，然后云端有新修改了，导致pull失败
最好的解决办法：撤销之前的修改，即执行以下命令：
```powershell
git restore .
```
然后再次执行上述pull命令即可。

#### 第二步：修改本地文件
修改仓库对应的本地文件就好了。

如果想查看自己做了哪些修改，可以使用以下命令：
- `git diff`：（对已有文件做了哪些修改？）
- `git status`：（有没有添加/删除文件？）

##### 操作提示：
如果使用默认安装的Git，执行`git diff`后，会进入一个Vim编辑器界面，里边的操作如下：
- `h/j/k/l`：左、下、上、右
- `<Ctrl>+U`：快速上翻
- `<Ctrl>+D>`：快速下翻
- `:q<Enter>`：退出（在里面先打一个英文冒号，再输入q，最后按回车）
至于其它的Vim操作，请自行学习，我用Vim几年了，Vim这玩意会了就真的爽。

#### 第三步：提交修改
使用`git diff`和`git status`检查好自己做的修改之后，完成以下几步操作：

##### 1. 将本地文件修改添加至本地暂存区
添加当前所有修改至暂存区：
```powershell
git add .
```
如果只想添加某个文件，将`.`改为对应文件路径即可。

##### 2. 将本地暂存区的修改提交到本地存储库
```powershell
git commit --message="提交信息"
```
将上面的`提交信息`修改成本次提交的提交信息即可。

提交信息有什么用？就是记录你做了什么修改的，在网页端会有显示，如下所示：

![](入门教程：Git基础.assets/commit-message-1.png)

所以，提交信息是会记录下来的，不会写就写个大概，不要随便乱写就行。

后续应该会有关于提交信息的规范，因为我看到大型的开源项目都是这么做的。

##### 3. 将本地的提交记录同步到远程存储库
```powershell
git push
```

##### 故障排除：出现冲突？
以下是模拟产生冲突和解决冲突的全过程：
```powershell
git clone https://github.com/MiyakoMeow/test-commit-conflict.git test-commit-conflict-1
git clone https://github.com/MiyakoMeow/test-commit-conflict.git test-commit-conflict-2
```

文件`README.md`有一行：`## test-commit-conflict`。

前往`./test-commit-conflict-1`，修改`README.md`文件中的该行至：`# test-commit-conflict-1`，提交成功，推送成功。

然后，前往`./test-commit-conflict-2`，修改`README.md`文件中的该行至：`# test-commit-conflict-2`，提交成功，推送时出现错误：
```
To github.com:MiyakoMeow/test-commit-conflict.git
 ! [rejected]        main -> main (fetch first)
error: failed to push some refs to 'github.com:MiyakoMeow/test-commit-conflict.git'
hint: Updates were rejected because the remote contains work that you do
hint: not have locally. This is usually caused by another repository pushing
hint: to the same ref. You may want to first integrate the remote changes
hint: (e.g., 'git pull ...') before pushing again.
hint: See the 'Note about fast-forwards' in 'git push --help' for details.
```
下面还给了提示，确实贴心。那就按照提示，`git pull`一下：
```
remote: Enumerating objects: 5, done.
remote: Counting objects: 100% (5/5), done.
remote: Compressing objects: 100% (2/2), done.
remote: Total 3 (delta 0), reused 3 (delta 0), pack-reused 0
Unpacking objects: 100% (3/3), 288 bytes | 1024 bytes/s, done.
From github.com:MiyakoMeow/test-commit-conflict
   b11fabb..31c1656  main       -> origin/main
Auto-merging README.md
CONFLICT (content): Merge conflict in README.md
Automatic merge failed; fix conflicts and then commit the result.
```
可以看到，对于`README.md`文件，两次修改到同一处，导致自动合并失败，需要我们手动解决冲突。

此时打开`README.md`文件，会发现以下自动生成的标记：
```
<<<<<<< HEAD
# test-commit-conflict-2
=======
# test-commit-conflict-1
>>>>>>> 31c1656b5734d598173a0b7d729fd2b62f1312c3
```
这又是什么意思呢？这是在此处产生冲突的两次修改的修改详情，可以看看上面。

接下来要做的事：将这些标记，替换成我们想要的内容，然后重新提交。

比如说，两次修改的内容我们都想要，并且按升序排序，那么将上面这个地方改成：
```
# test-commit-conflict-1
# test-commit-conflict-2
```
像这样，清理完所有自动生成的标记之后，直接添加、提交、推送三部曲就好了。
