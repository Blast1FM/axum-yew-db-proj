use std::ops::Deref;

use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use serde::{Deserialize, Serialize};
use reqwasm::http::Request;
use yew::prelude::*;

#[derive(Default, Clone)]
struct Query
{
  pub row_name: String,
  pub regexp: String,
  pub books: Vec<Book>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Book
{
    pub id: i32,
    pub author: String,
    pub name: String,
    pub publish_date: String,
    pub publisher: String,
    pub synopsis: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QueryResponse
{
  pub books: Vec<Book>,
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

  //TODO figure out callbacks and stop beinga monkey
  let on_click = {
    let state = query_state.clone();
    let _ = Callback::from(move |_:()| {
      let state = state.clone();
      wasm_bindgen_futures::spawn_local(async move 
      {
        let response = Request::get("http://localhost:8069/api/v1/books")
        .send()
        .await
        .unwrap()
        .json::<QueryResponse>()
        .await
        .unwrap();

        let mut query = state.deref().clone();
        query.books = Some(response.books).unwrap();
        state.set(query);
      })
    });
  };

    html! {
      <form>
        <TextInput name="Row name" handle_onchange={rowname_changed}/>
        <TextInput name="Regexp" handle_onchange={regexp_changed} />
        <button {on_click}> {"Submit Query"} </button> 
        <CustomButton label="Submit" />
        <p>{"Row name: "} {&query_state.row_name}</p>
        <p>{"Regexp: "} {&query_state.regexp}</p>
      </form>
    }
}

