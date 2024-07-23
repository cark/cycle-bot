use crate::{
    game::{
        editor::{Highlight, HighlightGizmos},
        physics::{coll_groups, ObjectGroup},
        spawn::wall::Wall,
    },
    mouse::MouseWorldCoords,
    AppSet,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Tool;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<PointerState>();
    app.enable_state_scoped_entities::<PointerState>();
    // app.insert_resource(Highlights::default());
    app.add_systems(
        Update,
        (clear_highlights, highlight_check)
            .chain()
            .in_set(AppSet::RecordInput)
            .run_if(in_state(PointerState::Highlight)),
    );
    app.add_systems(
        Update,
        highlight_walls
            .in_set(AppSet::Update)
            .run_if(in_state(PointerState::Highlight)),
    );
    app.add_systems(OnExit(PointerState::Highlight), clear_highlights);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(Tool = Tool::Pointer)]
pub enum PointerState {
    #[default]
    Highlight,
}

// #[derive(Debug, Default, Resource)]
// struct Highlights(HashSet<Entity>);

fn highlight_check(
    mouse_wc: Res<MouseWorldCoords>,
    rapier_context: Res<RapierContext>,
    // mut highlights: ResMut<Highlights>,
    mut cmd: Commands,
) {
    let Some(point) = mouse_wc.0 else { return };
    // let solid = true;
    let filter = QueryFilter::default().groups(coll_groups(u32::MAX, ObjectGroup::WALL));
    // warn!("{:?}", point);
    // if let Some((entity, projection)) = rapier_context.project_point(point, solid, filter) {
    //     println!(
    //         "Projected point on entity {:?}. Point projection: {}",
    //         entity, projection.point
    //     );
    //     println!(
    //         "Point was inside of the collider shape: {}",
    //         projection.is_inside
    //     );
    // }
    // highlights.0.clear();
    rapier_context.intersections_with_point(point, filter, |entity| {
        cmd.entity(entity).insert(Highlight);
        // highlights.0.insert(entity);
        // Callback called on each collider with a shape containing the point.
        // println!("The entity {:?} contains the point.", entity);
        // Return `false` instead if we want to stop searching for other colliders containing this point.
        true
    });
}

fn clear_highlights(mut cmd: Commands, q_highlights: Query<Entity, With<Highlight>>) {
    for entity in &q_highlights {
        cmd.entity(entity).remove::<Highlight>();
    }
}

fn highlight_walls(
    q_walls: Query<(&GlobalTransform, &Sprite), (With<Wall>, With<Highlight>)>,
    mut gizmos: Gizmos<HighlightGizmos>,
) {
    for (tr, sprite) in &q_walls {
        let tr = tr.compute_transform();
        gizmos.rect_2d(
            tr.translation.truncate(),
            tr.rotation.to_axis_angle().1,
            sprite.custom_size.unwrap(),
            Color::srgb(0.0, 1.0, 0.0),
        );
        // let rect = sprite.;
        // warn!("Got a wall !");
    }
}
