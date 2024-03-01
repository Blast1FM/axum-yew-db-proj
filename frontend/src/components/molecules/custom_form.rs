use std::ops::Deref;
use crate::components::atoms::text_input::TextInput;
use serde::{Deserialize, Serialize};
use reqwasm::http::Request;
use gloo::console::log;
use serde_json::to_string_pretty;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct Query
{
  pub row_name: String,
  pub regexp: String,
  pub books: Vec<Book>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
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

#[derive(Serialize, Deserialize)]
pub struct ActualQuery
{
  pub row_name:String,
  pub regexp:String,
}

#[derive(Properties, PartialEq)]
pub struct Props{
  pub onsubmit: Callback<Query>,
}

#[function_component(CustomForm)]
pub fn custom_form (/* props: &Props */) -> Html {

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

  //let form_onsubmit = props.onsubmit.clone();
  let cloned_query_state = query_state.clone();
  let onsubmit = Callback::from(move |event:SubmitEvent|{
    event.prevent_default();
    let state = cloned_query_state.clone();
    wasm_bindgen_futures::spawn_local(async move{
      let book_query = ActualQuery {row_name: state.row_name.to_owned(), regexp: state.regexp.to_owned()};
      let result = Request::new("http://[::1]:8069/api/v1/books")
      //TODO for some reason it says get or head request can't have a body? What?
      .method(reqwasm::http::Method::POST)
      .header("content-type", "application/json")      
      .body(serde_json::to_string(&book_query).unwrap())
      .send()
      .await
      .unwrap()
      .json::<QueryResponse>()
      .await;

      match result{
        Ok(result) => {
          let mut query = state.deref().clone();
          query.books = result.books;
          state.set(query);
        },
        Err(e) => {
          log!(format!("{e}"));
        ;}
      }    
    })
  });
    html! {
      <form {onsubmit}>
        <TextInput name="Row name" handle_onchange={rowname_changed}/>
        <TextInput name="Regexp" handle_onchange={regexp_changed} />
        <button> {"Submit Query"} </button> 
        <p>{"Row name: "} {&query_state.row_name}</p>
        <p>{"Regexp: "} {&query_state.regexp}</p>
        <pre> {to_string_pretty(&query_state.books).unwrap()} </pre>
      </form>
    }
}

