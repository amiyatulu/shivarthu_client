use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Props {
    id: String,
    children: Children,
}

#[function_component]
fn Container(props: &Props) -> Html {
    html! {
        <div id={props.id.clone()}>
           <h3>{"I am inside container"} </h3>
            { props.children.clone() }
        </div>
    }
}

#[function_component]
pub fn ContainerWithChildren() -> Html{
    html! {
        <Container id="container">
            <h4>{ "Hi" }</h4>
            <div>{ "Hello" }</div>
        </Container>
    }
}

