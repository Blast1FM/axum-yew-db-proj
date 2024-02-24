use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Query
{
  pub row_name: String,
  pub regexp: String
}


#[function_component(CustomForm)]
pub fn custom_form() -> Html {

  let rowname_changed = Callback::from(|row_name| 
  {
    gloo::console::log!("Row name changed to ", row_name);
  });

  let regexp_changed = Callback::from(|regexp|
  {
    gloo::console::log!("Regexp changed to ", regexp);
  });

    html! {
      <form>
        <TextInput name="Row name" handle_onchange={rowname_changed}/>
        <TextInput name="Regexp" handle_onchange={regexp_changed} />
        <CustomButton label="Submit" />
      </form>
    }
}

