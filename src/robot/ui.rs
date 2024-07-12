mod function;

use bevy::{asset::AssetPath, prelude::*};
use bevy_mod_picking::prelude::*;

use crate::{
    ui_components::{
        button_effect::ButtonColorCollection,
        input_area::{InputArea, InputAreaType},
    },
    FONT_PATH,
};

use self::function::{
    ConnectCoachActivator, ConnectCoachInputArea, ConnectMPUActivator, ConnectMPUInputArea,
    RobotUiFunctionPlugin, ToggleCppInputActivator, ToggleRustInputActivator,
};

/*
 * Part：插件
 */

pub(super) struct RobotUiPlugin;

impl Plugin for RobotUiPlugin {
    fn build(&self, app: &mut App) {
        // Setup ui
        app
            .add_systems(Startup, ui_robot_startup_system)
            // functions
            .add_plugins(RobotUiFunctionPlugin)
            // To be continued...
        ;
    }
}

/*
 * Part：常量
 */

const BUTTON_HEIGHT: f32 = 35.0;
const FONT_SIZE: f32 = 28.0;

const BUTTON_COLOR_COLLECTION: ButtonColorCollection = ButtonColorCollection::DEFAULT;
const INPUT_AREA_COLOR_COLLECTION: ButtonColorCollection =
    ButtonColorCollection::from_background(Color::ALICE_BLUE, Color::ALICE_BLUE, Color::BLUE);

/*
 * Part：根节点
 */

fn ui_robot_startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_asset_path = AssetPath::from_path(&FONT_PATH);

    let text_style = TextStyle {
        font: asset_server.load::<Font>(font_asset_path),
        font_size: FONT_SIZE,
        color: Color::BLACK,
    };
    // UI Root
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // 全尺寸根节点
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    // 使用CSS Grid
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::auto(), GridTrack::flex(1.0)],
                    grid_template_rows: vec![GridTrack::auto(), GridTrack::flex(1.0)],
                    ..default()
                },
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|node_parent| on_root(node_parent, text_style));
}

fn on_root(node_parent: &mut ChildBuilder<'_>, text_style: TextStyle) {
    // 顶部各部分状态栏
    node_parent
        .spawn(NodeBundle {
            style: Style {
                grid_column: GridPlacement::span(2),
                // 高200px
                height: Val::Px(200.0),
                // 使用CSS Grid，行侧4等分
                display: Display::Grid,
                grid_template_rows: vec![GridTrack::auto()],
                grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
                ..default()
            },
            ..default()
        })
        .with_children(|node_parent| on_top_status_area(node_parent, text_style.clone()));
    // 左侧信息栏
    node_parent
        .spawn(NodeBundle {
            style: Style {
                grid_row: GridPlacement::start_span(2, 1),
                grid_column: GridPlacement::start_span(1, 1),
                // 宽200px
                width: Val::Px(200.0),
                // 使用CSS Grid，行侧2等分
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::auto()],
                grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
                ..default()
            },
            ..default()
        })
        .with_children(|node_parent| on_left_info_area(node_parent, text_style.clone()));
    // 中间文字
    node_parent
        .spawn(NodeBundle {
            style: Style {
                grid_row: GridPlacement::start_span(2, 1),
                grid_column: GridPlacement::start_span(2, 1),
                ..default()
            },
            ..default()
        })
        .with_children(|node_parent| on_middle_text_area(node_parent, text_style.clone()));
}

/*
 * Part：左侧状态栏
 */

fn on_top_status_area(node_parent: &mut ChildBuilder<'_>, text_style: TextStyle) {
    // 一键启动
    node_parent.spawn(NodeBundle {
        style: Style {
            align_items: AlignItems::Default,
            justify_content: JustifyContent::Default,
            // 使用CSS Grid
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::auto()],
            grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
            ..default()
        },
        ..default()
    });
    // 教练机通信部分
    node_parent
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Default,
                justify_content: JustifyContent::Default,
                // 使用CSS Grid
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::auto()],
                grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                ..default()
            },
            ..default()
        })
        .with_children(|node_parent| on_coach_area(node_parent, text_style.clone()));
    // 全景相机部分
    node_parent
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Default,
                justify_content: JustifyContent::Default,
                ..default()
            },
            ..default()
        })
        .with_children(|node_parent| on_panorama_camera_area(node_parent, text_style.clone()));
    // 下位机部分
    node_parent
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Default,
                justify_content: JustifyContent::Default,
                ..default()
            },
            ..default()
        })
        .with_children(|node_parent| on_com_robot_area(node_parent, text_style.clone()));
    // MPU部分
    node_parent
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Default,
                justify_content: JustifyContent::Default,
                // 使用CSS Grid
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::auto()],
                grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                ..default()
            },
            ..default()
        })
        .with_children(|node_parent| on_com_mpu_area(node_parent, text_style));
}

fn on_coach_area(node_parent: &mut ChildBuilder<'_>, text_style: TextStyle) {
    // 状态
    node_parent
        .spawn(TextBundle::from_sections([
            TextSection::new("教练机：", text_style.clone()),
            TextSection::new(
                "未连接",
                TextStyle {
                    color: Color::RED,
                    ..text_style.clone()
                },
            ),
        ]))
        .insert(Style {
            height: Val::Px(30.0),
            grid_column: GridPlacement::span(2),
            ..default()
        });
    // IP地址输入框
    node_parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(140.0),
                height: Val::Px(BUTTON_HEIGHT),
                border: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                ..default()
            },
            border_color: INPUT_AREA_COLOR_COLLECTION.normal.border,
            background_color: INPUT_AREA_COLOR_COLLECTION.normal.background,
            transform: Transform::from_xyz(-100., 0., 1.),
            ..default()
        })
        .insert(InputArea {
            area_type: InputAreaType::IpAddress,
        })
        .insert(ConnectCoachInputArea)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "192.168.43.100",
                text_style.clone(),
            ));
        });
    // 教练机 连接按钮
    node_parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(60.0),
                height: Val::Px(BUTTON_HEIGHT),
                border: UiRect::all(Val::Px(3.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BUTTON_COLOR_COLLECTION.normal.border,
            background_color: BUTTON_COLOR_COLLECTION.normal.background,
            transform: Transform::from_xyz(-100., 0., 1.),
            ..default()
        })
        .insert(BUTTON_COLOR_COLLECTION)
        .insert(ConnectCoachActivator)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("连接", text_style));
        });
}

fn on_panorama_camera_area(node_parent: &mut ChildBuilder<'_>, text_style: TextStyle) {
    node_parent
        .spawn(TextBundle::from_sections([
            TextSection::new("全景相机：", text_style.clone()),
            TextSection::new(
                "未连接",
                TextStyle {
                    color: Color::RED,
                    ..text_style.clone()
                },
            ),
        ]))
        .insert(Style {
            height: Val::Px(30.0),
            ..default()
        });
}

fn on_com_robot_area(node_parent: &mut ChildBuilder<'_>, text_style: TextStyle) {
    node_parent
        .spawn(TextBundle::from_sections([
            TextSection::new("下位机：", text_style.clone()),
            TextSection::new(
                "未连接",
                TextStyle {
                    color: Color::RED,
                    ..text_style.clone()
                },
            ),
        ]))
        .insert(Style {
            height: Val::Px(30.0),
            ..default()
        });
}

fn on_com_mpu_area(node_parent: &mut ChildBuilder<'_>, text_style: TextStyle) {
    node_parent
        .spawn(TextBundle::from_sections([
            TextSection::new("MPU模块：", text_style.clone()),
            TextSection::new(
                "未连接",
                TextStyle {
                    color: Color::RED,
                    ..text_style.clone()
                },
            ),
        ]))
        .insert(Style {
            height: Val::Px(30.0),
            grid_column: GridPlacement::span(2),
            ..default()
        });
    // MPU Com口输入框
    node_parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(140.0),
                height: Val::Px(BUTTON_HEIGHT),
                border: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                ..default()
            },
            border_color: INPUT_AREA_COLOR_COLLECTION.normal.border,
            background_color: INPUT_AREA_COLOR_COLLECTION.normal.background,
            transform: Transform::from_xyz(-100., 0., 1.),
            ..default()
        })
        .insert(InputArea {
            area_type: InputAreaType::Number,
        })
        .insert(ConnectMPUInputArea)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("5", text_style.clone()));
        });
    // MPU 连接按钮
    node_parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(60.0),
                height: Val::Px(BUTTON_HEIGHT),
                border: UiRect::all(Val::Px(3.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BUTTON_COLOR_COLLECTION.normal.border,
            background_color: BUTTON_COLOR_COLLECTION.normal.background,
            transform: Transform::from_xyz(-100., 0., 1.),
            ..default()
        })
        .insert(BUTTON_COLOR_COLLECTION)
        .insert(ConnectMPUActivator)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("连接", text_style));
        });
}

/*
 * Part：中间显示栏
 */
#[derive(Component)]
pub struct ShowSelectedText;

fn on_middle_text_area(node_parent: &mut ChildBuilder<'_>, text_style: TextStyle) {
    node_parent.spawn((
        TextBundle::from_sections([
            TextSection::new("当前选中：", text_style.clone()),
            TextSection::new(
                "无",
                TextStyle {
                    color: Color::BLUE,
                    ..text_style.clone()
                },
            ),
        ])
        .with_style(Style {
            height: Val::Px(30.0),
            ..Default::default()
        }),
        ShowSelectedText,
    ));
}

/*
 * Part：右侧控制栏
 */

fn on_left_info_area(node_parent: &mut ChildBuilder<'_>, text_style: TextStyle) {
    node_parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(150.0),
                height: Val::Px(BUTTON_HEIGHT),
                border: UiRect::all(Val::Px(3.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BUTTON_COLOR_COLLECTION.normal.border,
            background_color: BUTTON_COLOR_COLLECTION.normal.background,
            transform: Transform::from_xyz(-100., 0., 1.),
            ..default()
        })
        .insert(BUTTON_COLOR_COLLECTION)
        .insert(ToggleRustInputActivator)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Rust输入测试", text_style.clone()));
        });
    node_parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(150.0),
                height: Val::Px(BUTTON_HEIGHT),
                border: UiRect::all(Val::Px(3.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BUTTON_COLOR_COLLECTION.normal.border,
            background_color: BUTTON_COLOR_COLLECTION.normal.background,
            transform: Transform::from_xyz(-100., 0., 1.),
            ..default()
        })
        .insert(BUTTON_COLOR_COLLECTION)
        .insert(ToggleCppInputActivator)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Cpp输入测试", text_style.clone()));
        });
}
