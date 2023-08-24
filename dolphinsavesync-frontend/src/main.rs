mod app;
mod components;
mod views;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
