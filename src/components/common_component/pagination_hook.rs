use yew::prelude::*;

pub const DOTS: &str = "...";

#[derive(Clone, PartialEq)]
pub enum PageRange {
    Value(u64),
    DOTS
}

fn range(start: u64, end: u64) -> Vec<PageRange> {
    (start..=end).map(|x| PageRange::Value(x)).collect()
}

#[derive(Clone)]
pub struct PaginationHookProps {
    pub total_count: u64,
    pub page_size: u64,
    pub sibling_count: u64,
    pub current_page: u64,
}

#[hook]
pub fn use_pagination(props: PaginationHookProps) -> Vec<PageRange> {
    let props_clone = props.clone();

    let pagination_range = use_memo(move |_| {
        let total_page_count = (props_clone.total_count + props_clone.page_size - 1) / props_clone.page_size;
        // gloo::console::log!(total_page_count);

        let total_page_numbers = props_clone.sibling_count + 5;

        // gloo::console::log!(total_page_numbers);

        if total_page_numbers >= total_page_count {
            return range(1, total_page_count);
        }

        let left_sibling_index = props_clone.current_page.saturating_sub(props_clone.sibling_count);
        let right_sibling_index = (props_clone.current_page + props_clone.sibling_count).min(total_page_count);

        let should_show_left_dots = left_sibling_index > 2;
        let should_show_right_dots = right_sibling_index < total_page_count - 2;

        let first_page_index = 1;
        let last_page_index = total_page_count;

        if !should_show_left_dots && should_show_right_dots {
            let left_item_count = 3 + 2 * props_clone.sibling_count;
            let left_range = range(1, left_item_count);

            return [&left_range[..], &[PageRange::DOTS], &[PageRange::Value(total_page_count)]].concat();
        }

        if should_show_left_dots && !should_show_right_dots {
            let right_item_count = 3 + 2 * props_clone.sibling_count;
            let right_range = range(total_page_count.saturating_sub(right_item_count) + 1, total_page_count);

            return [&[PageRange::Value(first_page_index)], &[PageRange::DOTS], &right_range[..]].concat();
        }

        if should_show_left_dots && should_show_right_dots {
            let middle_range = range(left_sibling_index, right_sibling_index);

            return [&[PageRange::Value(first_page_index)], &[PageRange::DOTS], &middle_range[..], &[PageRange::DOTS], &[PageRange::Value(last_page_index)]].concat();
        }

        vec![]
    }, vec![
        props.total_count,
        props.page_size,
        props.sibling_count,
        props.current_page,
    ]);

    pagination_range.to_vec()
}
