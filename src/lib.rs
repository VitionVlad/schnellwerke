use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{engine::render::render::render_loop, loop_tick::save_load::{load_progress, load_settings}};

mod engine;
mod init;
mod loop_tick;
mod app_state;

async fn main_async() {
    let (mut eng, mut state) = init::create_app(false, false).await;

    let _ = load_settings("settings.json", &mut eng, &mut state);
    let _ = load_progress("save.json", &mut state);

    render_loop(Closure::new(move || {
      eng.work();
      loop_tick::soundwork::soundwork(&mut eng, &mut state);
      loop_tick::control_handle::control_handle(&mut eng, &mut state);
      loop_tick::tick::tick(&mut eng, &mut state);
      loop_tick::per_select_tick::per_select_tick(&mut eng, &mut state);
      loop_tick::handle_scene::handle_scene(&mut eng, &mut state);
      loop_tick::menu_handle::menu_handle(&mut eng, &mut state);
      state.framecnt+=1;
    }));
}

#[wasm_bindgen]
pub fn main() {
    spawn_local(async {
        main_async().await;
    });
}