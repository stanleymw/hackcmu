use bevy::ecs::{
    error::Result,
    resource::Resource,
    system::{Local, Query, Res, ResMut},
};
use bevy_egui::{
    EguiContexts,
    egui::{self, RichText},
};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

use crate::{CurrentLevel, LevelIndex, wasm::CodeBuffer};

#[derive(Resource)]
pub struct ReferenceOpen(pub bool);

pub fn reference_ui(
    mut contexts: EguiContexts,
    mut is_open: ResMut<ReferenceOpen>,
    mut commonmark_cache: Local<CommonMarkCache>,
    level_query: Query<(&CodeBuffer, &LevelIndex)>,
    current_level: Res<CurrentLevel>,
) -> Result {
    egui::Window::new("Reference")
        .open(&mut is_open.0)
        .show(contexts.ctx_mut()?, |ui| {
            for (buf, idx) in level_query.iter() {
                if idx.0 != current_level.index {
                    continue;
                }
                let viewer = CommonMarkViewer::new();
                viewer.show(ui, &mut commonmark_cache, &buf.reference);
            }
        });
    Ok(())
}

const EX_MARKDOWN: &'static str = "
# big header
## small header

### smaller header

- first item
- second item

```python
def main():
    return 5
```
";
