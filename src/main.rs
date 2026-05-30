mod sgl;

use crate::sgl::{
    sgl::{InputEvent, SGLApp, SGLContext},
    types::Scene,
};

struct App {}

impl SGLApp for App {
    fn update(&mut self, scene: &mut Scene) -> Option<Scene> {
        None
    }

    fn on_input(&mut self, event: InputEvent, scene: &mut Scene) {}
}

fn main() {
    let scene = Scene::new();
    let app = App {};

    let context = SGLContext::new(scene, app);
    context.run();
}
