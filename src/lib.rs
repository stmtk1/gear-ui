use yew::{
    Callback,
    function_component,
    prelude::{ Html, html, },
    Properties, 
};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub value: String,
    pub onchange: Callback<yew::html::onchange::Event>,
}

#[function_component]
pub fn Input(props: &Props) -> Html {
    let Props {value, onchange} = props.clone();
    html! { <input {value} {onchange} /> }
}