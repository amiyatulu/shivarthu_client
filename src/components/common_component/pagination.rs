use crate::components::common_component::pagination_hook::{
    use_pagination, PageRange, PaginationHookProps, DOTS,
};
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

    let on_next = {
        let current_page = current_page.clone();
        let on_page_change = on_page_change.clone();
        move |_| {
            let value = current_page + 1;
            on_page_change.emit(value);
        }
    };

    let on_previous = {
        let current_page = current_page.clone();
        let on_page_change = on_page_change.clone();
        move |_| {
            let value = current_page - 1;
            on_page_change.emit(value);
        }
    };

    let last_page_range = pagination_range.get(pagination_range.len() - 1).unwrap();

    let pagination_values = pagination_range
        .iter()
        .map(|value| {
            html! {
                if *value == PageRange::DOTS {
                    <li class="page-item"><button class="page-link">{DOTS}</button></li>
                } else if let PageRange::Value(page) = *value {
                    <li class="page-item"><button class="page-link" onclick={
                        let on_page_change = on_page_change.clone();
                        move |_| {
                            on_page_change.emit(page);
                    }}>{page}</button></li>
                }

            }
        })
        .collect::<Html>();

    if current_page == 0 || pagination_range.len() < 2 {
        html! {
            <>
            </>
        }
    } else {
        html! {
            <>
                <nav>
                 <ul class="pagination">
                    <li class="page-item"><button class="page-link" disabled={current_page == 1}  onclick={on_previous}>{"Previous"}</button></li>
                    {pagination_values}
                    if let PageRange::Value(last_page) = last_page_range {
                        <li class="page-item"><button class="page-link" disabled={current_page == *last_page} onclick={on_next}>{"Next"}</button></li>
                    }
                </ul>
                </nav>
            </>
        }
    }
}
