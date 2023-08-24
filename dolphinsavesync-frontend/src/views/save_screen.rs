use yew::{function_component, Html, html, Properties};
use crate::components::save_slot::SaveSlot;

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[function_component(SaveScreenView)]
pub fn save_scree_view() -> Html {
    let saves: Vec<i32> = (1..=5).collect();

    html! {
        <div>
            {
                saves.into_iter().map(|_save| {
                    html! {
                        <SaveSlot />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}