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

  let query_state = use_state(|| Query{row_name:"null".to_owned(),regexp:"null".to_owned()});
  let cloned_query_state = query_state.clone();
  let rowname_changed = Callback::from(move |rowname| 
  {
    //todo fix
    //query_state.row_name.set(value);
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

