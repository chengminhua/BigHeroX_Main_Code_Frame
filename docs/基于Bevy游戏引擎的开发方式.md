# BigHeroX RoboCup：Bevy开发
- Bevy官网：https://bevyengine.org

> ECS介绍：</br>
> 漫谈Entity Component System (ECS) - Rouder的文章 - 知乎</br>
> https://zhuanlan.zhihu.com/p/270927422

- Entity：实体
- Component：组件
- System：系统

以下部分将尽可能详细介绍Bevy的使用方式。

## Bevy中的ECS

```rust
// 引入Bevy中的主要成员
use bevy::prelude::*;

// main函数
fn main() {
    App::new()
        // 添加系统至Startup阶段（Schedule）
        // Startup阶段只会运行一次，因此该系统只会运行一次
        .add_systems(Startup, add_entities)
        // 添加下面定义的插件
        .add_plugins(MyPlugin)
        .run();
}

/*
 * 插件（Plugin）部分
 */
struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app
            // 添加系统至Update阶段
            // Update阶段会在程序渲染一帧时执行一次
            .add_systems(Update, move_x)
            // 添加系统至FixedUpdate阶段
            // FixedUpdate阶段有固定的执行间隔（默认为每秒64次）
            .add_systems(FixedUpdate, move_y)
            // 当然Bevy也不只有这几个阶段
            ;
    }
}

// 如果想让一个类型作为组件被附加到实体上，先为该类型实现Component特征
#[derive(Component)]
struct MoveX;
// 可以往组件里面塞值
#[derive(Component)]
struct MoveY {
    id: usize,
    repeat_time: usize,
}

// 用于添加实体的系统
// 会被添加至Startup阶段
fn add_entities(
    // 命令
    mut commands: Commands,
) {
    // 生成实体，为实体附加组件
    commands.spawn(MoveX);
}
```