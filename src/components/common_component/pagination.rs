use crate::components::common_component::pagination_hook::{
    use_pagination, PageRange, PaginationHookProps, DOTS,
};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PaginationProps {
    pub on_page_change: Callback<u64>,
    pub total_count: u64,
    pub sibling_count: u64,
    pub current_page: u64,
    pub page_size: u64,
    pub class_name: Option<String>,
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let on_page_change = props.on_page_change.clone();
    let total_count = props.total_count.clone();
    let sibling_count = props.sibling_count.clone();
    let current_page = props.current_page.clone();
    let page_size = props.page_size.clone();
    let class_name = props.class_name.clone();

    let pagination_range = use_pagination(PaginationHookProps {
        total_count,
        page_size,
        sibling_count,
        current_page,
    });

    if current_page == 0 || pagination_range.len() < 2 {
        html! {
            <>
            </>
        }
      } else {
        html! {
            <>
            </>
        }
      }
    

    
}
