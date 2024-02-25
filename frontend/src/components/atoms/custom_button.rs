use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub request_uri: String,
    //pub handle_onsubmit: Callback<()>,
}

#[function_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
    /* let handle_onsubmit = props.handle_onsubmit.clone();
    let onsubmit = Callback::from(move |event:FocusEvent|{
      event.prevent_default();
      
    }); */
    html! {
      <button>{&props.label}</button>
    }
}
