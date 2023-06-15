use yew::{
    Callback,
    function_component,
    prelude::{ Html, html, },
    Properties, 
};

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    pub value: String,
    pub oninput: Callback<web_sys::InputEvent>,
}

#[function_component]
pub fn Input(props: &InputProps) -> Html {
    let InputProps {value, oninput} = props.clone();
    html! { <input type={"text"} {value} {oninput} /> }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub value: String,
    pub onclick: Callback<web_sys::MouseEvent>,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let ButtonProps {value, onclick} = props.clone();
    html! { <button {onclick}>{value}</button> }
}

#[derive(Properties, PartialEq, Clone)]
pub struct LabelProps {
    pub value: String,
}

#[function_component]
pub fn Label(props: &LabelProps) -> Html {
    let LabelProps {value } = props.clone();
    html! { <label>{value}</label> }
}
