use yew::{
    Callback,
    function_component,
    prelude::{ Html, html, },
    Properties, 
};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub value: String,
    pub oninput: Callback<web_sys::InputEvent>,
}

#[function_component]
pub fn Input(props: &Props) -> Html {
    let Props {value, oninput} = props.clone();
    html! { <input type={"text"} {value} {oninput} /> }
}