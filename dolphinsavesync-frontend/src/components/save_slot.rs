use yew::{function_component, Html, html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[function_component(SaveSlot)]
pub fn save_slot() -> Html {
    html! {
        <div>
            {"Save"}
        </div>
    }
}