use bevy::{
    ecs::{
        error::Result,
        event::EventWriter,
        resource::Resource,
        system::{Local, Query, Res, ResMut},
    },
    math::Vec2,
    reflect::Reflect,
    window::{MonitorSelection, WindowPosition},
};
use bevy_egui::{
    EguiContexts,
    egui::{self, Align2, Pos2, RichText, ViewportInfo, emath},
    helpers::vec2_into_egui_vec2,
};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

use crate::{CurrentLevel, HasWon, LevelIndex, game::GameResetEvent, wasm::CodeBuffer};

#[derive(Resource)]
pub struct ReferenceOpen(pub bool);

pub fn win_screen(
    mut contexts: EguiContexts,
    mut has_won: ResMut<HasWon>,
    mut current_lvl: ResMut<CurrentLevel>,
    mut events: EventWriter<GameResetEvent>,
) -> Result {
    if has_won.0 {
        egui::Window::new("You Win!")
            .anchor(Align2::CENTER_CENTER, emath::vec2(0.0, 0.0))
            .show(contexts.ctx_mut()?, |ui| {
                if ui.button("Next Level").clicked() {
                    current_lvl.index = current_lvl.index.wrapping_add(1);
                    events.write(GameResetEvent);
                    has_won.0 = false;
                }
            });
    }

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
