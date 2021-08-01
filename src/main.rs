pub mod vec2;
pub mod snake;
pub mod gamestate;
pub mod ui;

fn main() {
    let mut ui = ui::Ui::new();
    ui.start();
}
