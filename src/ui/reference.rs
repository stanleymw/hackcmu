use bevy::ecs::{error::Result, resource::Resource, system::{Local, ResMut}};
use bevy_egui::{egui::{self, RichText}, EguiContexts};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};


#[derive(Resource)]
pub struct ReferenceOpen(pub bool);

pub fn reference_ui(
    mut contexts: EguiContexts,
    mut is_open: ResMut<ReferenceOpen>,
    mut commonmark_cache: Local<CommonMarkCache>,
) -> Result {
    
    egui::Window::new("Reference").open(&mut is_open.0).show(contexts.ctx_mut()?, |ui| {
        let viewer = CommonMarkViewer::new();
        viewer.show(ui, &mut commonmark_cache, EX_MARKDOWN);
    });
    Ok(())
}

const EX_MARKDOWN: &'static str =
"
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