use yew::{function_component, Html, html};
use crate::views::save_screen::SaveScreenView;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <SaveScreenView />
        </div>
    }
}