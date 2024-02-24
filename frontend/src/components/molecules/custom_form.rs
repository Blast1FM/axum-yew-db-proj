use std::ops::Deref;

use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Default, Clone)]
struct Query
{
  pub row_name: String,
  pub regexp: String,
}


#[function_component(CustomForm)]
pub fn custom_form() -> Html {

  let query_state = use_state(|| Query::default());
  let cloned_query_state = query_state.clone();
  let rowname_changed = Callback::from(move |rowname| 
  {
    let mut data = cloned_query_state.deref().clone();
    data.row_name = rowname;
    cloned_query_state.set(data);
  });

  let cloned_query_state = query_state.clone();
  let regexp_changed = Callback::from(move |regexp|
  {
    let mut data = cloned_query_state.deref().clone();
    data.regexp = regexp;
    cloned_query_state.set(data);
  });

    html! {
      <form>
        <TextInput name="Row name" handle_onchange={rowname_changed}/>
        <TextInput name="Regexp" handle_onchange={regexp_changed} />
        <CustomButton label="Submit" />
        <p>{"Row name: "} {&query_state.row_name}</p>
        <p>{"Regexp: "} {&query_state.regexp}</p>
      </form>
    }
}

