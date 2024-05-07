use bevy_ecs::system::Query;

use crate::ecs::components::Window;

/// Requests a rerender for each window.
pub fn rerender_request_system(query: Query<&Window>) {
    for window in query.iter() {
        window.request_rerender();
    }
}
