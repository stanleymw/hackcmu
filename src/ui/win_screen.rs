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

use crate::{CurrentLevel, HasWon, LevelIndex, wasm::CodeBuffer};

#[derive(Resource)]
pub struct ReferenceOpen(pub bool);

pub fn win_screen(
    mut contexts: EguiContexts,
    mut has_won: ResMut<HasWon>,
    mut current_lvl: ResMut<CurrentLevel>,
) -> Result {
    if has_won.0 {
        egui::Window::new("You Win!").show(contexts.ctx_mut()?, |ui| {
            if ui.button("Next Level").clicked() {
                current_lvl.index = current_lvl.index.wrapping_add(1);
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
