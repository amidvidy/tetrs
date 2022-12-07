use cursive::views::LinearLayout;

use log::LevelFilter;

mod game;

fn main() {
    simple_logging::log_to_file("test.log", LevelFilter::Info).unwrap();

    log::info!("Hello, world!");

    let mut siv = cursive::default();

    let game = game::Game::new();

    siv.set_fps(1);

    siv.add_global_callback('q', cursive::Cursive::quit);
    siv.add_global_callback('~', cursive::Cursive::toggle_debug_console);
    siv.add_layer(
        LinearLayout::vertical().child(game), //.child(DebugView::new()),
    );

    siv.run();
}
