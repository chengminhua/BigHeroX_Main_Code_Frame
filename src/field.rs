//! 场地表示，包括球与球员

pub mod data;
pub use data::FieldData;

use std::f32::consts::PI;

use bevy::{
    asset::AssetPath,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_mod_picking::prelude::*;

use crate::FONT_PATH;

/*
 * Part：插件
 */

pub(super) struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FieldData::default())
            .add_event::<ClickEvent>()
            .add_systems(Startup, draw_field_system)
            .add_systems(Startup, draw_info_system)
            .add_systems(
                Update,
                receive_event_system.run_if(on_event::<ClickEvent>()),
            );
    }
}

/*
 * Part：标识
 */

/// 球员和球的图形实体
#[derive(Component)]
pub(super) struct InfoElement;

/*
 * Part：点击事件
 */
#[derive(Debug, Event)]
pub struct ClickEvent {
    /// (X, Z) in field
    pub pos: Vec2,
}

impl From<ListenerInput<Pointer<Down>>> for ClickEvent {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        let down = &event.event;
        let target = event.target;
        let down_hit = &down.hit;
        let down_hit_position = down_hit.position.unwrap();
        info!(
            "Clicked: target: {:?} hit_pos: {:.2} {:.2} {:.2}",
            target, down_hit_position.x, down_hit_position.y, down_hit_position.z,
        );
        ClickEvent {
            pos: Vec2::new(down_hit_position.x, down_hit_position.y),
        }
    }
}

fn receive_event_system(mut events: EventReader<ClickEvent>) {
    for _event in events.read() {}
}

/*
 * Part：场地生成
 */

const LINE_WIDTH: f32 = 0.1;

const FIELD_COLOR: Color = Color::DARK_GREEN;
const LINE_COLOR: Color = Color::WHITE;
const BALL_COLOR: Color = Color::YELLOW;
const TEAM_CYAN_COLOR: Color = Color::CYAN;
const TEAM_VIOLET_COLOR: Color = Color::VIOLET;

const LINE_HEIGHT: f32 = 0.001;
const BALL_HEIGHT: f32 = 0.002;
const ROBOT_HEIGHT: f32 = 0.01;

const ROUND_SPILT_COUNT: f32 = 100.0;

fn draw_field_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    field_data: Res<FieldData>,
) {
    commands
        .spawn((
            TransformBundle::from_transform(Transform::default()),
            PickableBundle::default(),
            On::<Pointer<Down>>::send_event::<ClickEvent>(),
        ))
        .with_children(|parent| on_root(parent, &mut meshes, &mut materials, &field_data));
}

fn on_root(
    commands: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    field_data: &Res<FieldData>,
) {
    let FieldData {
        field_size,
        gate_size,
        ..
    } = field_data.as_ref().to_owned();
    let (field_length, field_width) = <(f32, f32)>::from(field_size);
    let (gate_depth, _gate_width, _gate_height) = <(f32, f32, f32)>::from(gate_size);
    // 绿色场地
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::from_size(Vec2::new(
                    field_length + (gate_depth * 2.0) + 1.0,
                    field_width + 1.5,
                )))),
                material: materials.add(FIELD_COLOR),
                transform: Transform::default(),
                ..Default::default()
            },
            PickableBundle::default(),
        ))
        .with_children(|parent| on_field(parent, meshes, materials, field_data));
}

fn on_field(
    commands: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    field_data: &Res<FieldData>,
) {
    let FieldData {
        field_size,
        penalty_area_size,
        goal_area_size,
        center_circle_radius,
        corner_circle_radius,
        gate_size,
        ..
    } = field_data.as_ref().to_owned();
    let (field_length, field_width) = <(f32, f32)>::from(field_size);
    let (penalty_area_depth, penalty_area_width) = <(f32, f32)>::from(penalty_area_size);
    let (goal_area_depth, goal_area_width) = <(f32, f32)>::from(goal_area_size);
    let (gate_depth, gate_width, _gate_height) = <(f32, f32, f32)>::from(gate_size);
    // 边线
    let mesh_field_length = meshes.add(Rectangle::from_size(Vec2::new(
        field_length + LINE_WIDTH,
        LINE_WIDTH,
    )));
    let mesh_field_width = meshes.add(Rectangle::from_size(Vec2::new(
        LINE_WIDTH,
        field_width - LINE_WIDTH,
    )));
    let line_material = materials.add(LINE_COLOR);
    for xy_ratio in [1.0, -1.0] {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(mesh_field_length.clone()),
                material: line_material.clone(),
                transform: Transform::from_xyz(0.0, xy_ratio * (field_width / 2.), LINE_HEIGHT),
                ..default()
            },
            PickableBundle::default(),
        ));
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(mesh_field_width.clone()),
                material: line_material.clone(),
                transform: Transform::from_xyz(xy_ratio * (field_length / 2.), 0.0, LINE_HEIGHT),
                ..default()
            },
            PickableBundle::default(),
        ));
    }
    // 中线
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(mesh_field_width),
            material: line_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, LINE_HEIGHT),
            ..default()
        },
        PickableBundle::default(),
    ));
    // 大禁区
    spawn_penalty(
        commands,
        meshes,
        line_material.clone(),
        field_data.as_ref().to_owned(),
        penalty_area_depth,
        penalty_area_width,
    );
    // 小禁区
    spawn_penalty(
        commands,
        meshes,
        line_material.clone(),
        field_data.as_ref().to_owned(),
        goal_area_depth,
        goal_area_width,
    );
    // 球门
    let gate_depth_shape =
        Rectangle::from_size(Vec2::new(f32::abs(gate_depth) + LINE_WIDTH, LINE_WIDTH));
    let gate_length_mesh = meshes.add(gate_depth_shape);
    let gate_width_shape =
        Rectangle::from_size(Vec2::new(LINE_WIDTH, f32::abs(gate_width) - LINE_WIDTH));
    let gate_width_mesh = meshes.add(gate_width_shape);
    for (x_ratio, color) in [1.0, -1.0]
        .into_iter()
        .zip([TEAM_VIOLET_COLOR, TEAM_CYAN_COLOR])
    {
        let gate_line_material = materials.add(color);
        for y_ratio in [1.0, -1.0] {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(gate_length_mesh.clone()),
                    material: gate_line_material.clone(),
                    transform: Transform::from_xyz(
                        x_ratio * ((field_length / 2.) + (gate_depth / 2.)),
                        y_ratio * (gate_width / 2.),
                        LINE_HEIGHT + 0.001,
                    ),
                    ..default()
                },
                PickableBundle::default(),
            ));
        }
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(gate_width_mesh.clone()),
                material: gate_line_material.clone(),
                transform: Transform::from_xyz(
                    x_ratio * ((field_length / 2.) + gate_depth),
                    0.0,
                    LINE_HEIGHT + 0.001,
                ),
                ..default()
            },
            PickableBundle::default(),
        ));
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(gate_width_mesh.clone()),
                material: gate_line_material,
                transform: Transform::from_xyz(
                    x_ratio * (field_length / 2.),
                    0.0,
                    LINE_HEIGHT + 0.001,
                ),
                ..default()
            },
            PickableBundle::default(),
        ));
    }
    // 中圈
    std::iter::successors(Some(0.0), |angle| {
        Some(angle + (PI / ROUND_SPILT_COUNT)).filter(|angle| *angle < (2.0 * PI))
    })
    .map(|angle| (angle, f32::sin_cos(angle)))
    .for_each(|(angle, (angle_sin, angle_cos))| {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(Rectangle::from_size(Vec2::new(LINE_WIDTH, LINE_WIDTH))),
                ),
                material: line_material.clone(),
                transform: Transform::from_xyz(
                    center_circle_radius * angle_cos,
                    center_circle_radius * angle_sin,
                    LINE_HEIGHT,
                )
                .with_rotation(Quat::from_rotation_z(angle)),
                ..default()
            },
            PickableBundle::default(),
        ));
    });
    // 四角1/4圆弧
    [1.0, -1.0]
        .into_iter()
        .flat_map(|x_ratio| [1.0, -1.0].map(|y_ratio| (x_ratio, y_ratio)))
        .for_each(|(x_ratio, y_ratio)| {
            let start_angle = match (x_ratio, y_ratio) {
                (x_ratio, y_ratio) if x_ratio > 0.0 && y_ratio > 0.0 => 1.0 * PI,
                (_x_ratio, y_ratio) if y_ratio > 0.0 => 1.5 * PI,
                (x_ratio, _y_ratio) if x_ratio > 0.0 => 0.5 * PI,
                (_x_ratio, _y_ratio) => 0.0 * PI,
            };
            std::iter::successors(Some(0.0), |angle| {
                Some(angle + (PI / ROUND_SPILT_COUNT)).filter(|angle| *angle < (0.5 * PI))
            })
            .map(|angle| start_angle + angle)
            .map(|angle| (angle, f32::sin_cos(angle)))
            .for_each(|(angle, (angle_sin, angle_cos))| {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(
                            meshes.add(Rectangle::from_size(Vec2::new(LINE_WIDTH, LINE_WIDTH))),
                        ),
                        material: line_material.clone(),
                        transform: Transform::from_xyz(
                            x_ratio * (field_length / 2.0) + corner_circle_radius * angle_cos,
                            y_ratio * (field_width / 2.0) + corner_circle_radius * angle_sin,
                            LINE_HEIGHT,
                        )
                        .with_rotation(Quat::from_rotation_z(angle)),
                        ..default()
                    },
                    PickableBundle::default(),
                ));
            });
        });
}

fn spawn_penalty(
    commands: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    line_material: Handle<ColorMaterial>,
    field_data: FieldData,
    penalty_depth: f32,
    penalty_width: f32,
) {
    let FieldData { field_size, .. } = field_data.to_owned();
    let (field_length, _field_width) = <(f32, f32)>::from(field_size);

    let penalty_depth_shape =
        Rectangle::from_size(Vec2::new(f32::abs(penalty_depth) + LINE_WIDTH, LINE_WIDTH));
    let penalty_length_mesh = meshes.add(penalty_depth_shape);
    let penalty_width_shape =
        Rectangle::from_size(Vec2::new(LINE_WIDTH, f32::abs(penalty_width) - LINE_WIDTH));
    let penalty_width_mesh = meshes.add(penalty_width_shape);
    for x_ratio in [1.0, -1.0] {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(penalty_width_mesh.clone()),
                material: line_material.clone(),
                transform: Transform::from_xyz(
                    x_ratio * ((field_length / 2.) - penalty_depth),
                    0.0,
                    LINE_HEIGHT,
                ),
                ..default()
            },
            PickableBundle::default(),
        ));
        for y_ratio in [1.0, -1.0] {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(penalty_length_mesh.clone()),
                    material: line_material.clone(),
                    transform: Transform::from_xyz(
                        x_ratio * ((field_length / 2.) - (penalty_depth / 2.)),
                        y_ratio * (penalty_width / 2.),
                        LINE_HEIGHT,
                    ),
                    ..default()
                },
                PickableBundle::default(),
            ));
        }
    }
}

fn draw_info_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 球
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(bevy::math::prelude::Circle::new(0.2))),
            material: materials.add(BALL_COLOR),
            transform: Transform::from_xyz(0.0, 0.0, BALL_HEIGHT),
            ..Default::default()
        })
        .insert(InfoElement)
        .insert(PickableBundle::default());
    // 球员
    for player_index in 0..5 {
        let tran = Transform::from_xyz(-(player_index + 1) as f32, 0.0, ROBOT_HEIGHT);
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(bevy::math::prelude::Circle::new(0.2))),
                material: materials.add(TEAM_CYAN_COLOR),
                transform: tran,
                ..Default::default()
            })
            .insert(InfoElement)
            .insert(PickableBundle::default());
        let tran = Transform::from_xyz(-(player_index + 1) as f32, 0.0, ROBOT_HEIGHT + 0.001);
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{player_index}"),
                TextStyle {
                    font: asset_server.load(AssetPath::from_path(&FONT_PATH)),
                    font_size: 48.0,
                    color: Color::BLACK,
                },
            ),
            transform: tran.with_scale(Vec3::splat(0.01)),
            ..Default::default()
        });
    }
}
