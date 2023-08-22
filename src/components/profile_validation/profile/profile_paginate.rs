use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PaginationProps {
    pub posts_per_page: u64,
    pub total_posts: u64,
    pub paginate: Callback<u64>,
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps)-> Html {
    

    html!{
        <>
        </>
    }
}

