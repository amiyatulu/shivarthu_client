use yew::prelude::*;
use crate::components::accounts::account_address_view::AccountAddressView;
use crate::components::accounts::signout_view::SignOutView;
use crate::components::balance::balance::Balance;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <>
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark" aria-label="Offcanvas navbar large">
                <div class="container-fluid">
                <a class="navbar-brand" href="#">{"Shivarthu"}</a>
                <button class="navbar-toggler" type="button" data-bs-toggle="offcanvas" data-bs-target="#offcanvasNavbar2" aria-controls="offcanvasNavbar2">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="offcanvas offcanvas-end text-bg-dark" tabindex="-1" id="offcanvasNavbar2" aria-labelledby="offcanvasNavbar2Label">
                    <div class="offcanvas-header">
                    <h5 class="offcanvas-title" id="offcanvasNavbar2Label">{"Offcanvas"}</h5>
                    <button type="button" class="btn-close btn-close-white" data-bs-dismiss="offcanvas" aria-label="Close"></button>
                    </div>
                    <div class="offcanvas-body">
                    <ul class="navbar-nav justify-content-end flex-grow-1 pe-3">
                        <li class="nav-item">
                        <a class="nav-link active" aria-current="page" href="#">{"Home"}</a>
                        </li>
                        <SignOutView/>
                        <li class="nav-item dropdown">
                        <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                            {"Dropdown"}
                        </a>
                        <ul class="dropdown-menu">
                            <li><a class="dropdown-item" href="#">{"Action"}</a></li>
                            <li><a class="dropdown-item" href="#">{"Another action"}</a></li>
                            <li>
                            <hr class="dropdown-divider"/>
                            </li>
                            <li><a class="dropdown-item" href="#">{"Something else here"}</a></li>
                        </ul>
                        </li>
                        <li class="nav-item">
                        <span class="nav-link active" aria-current="page" ><AccountAddressView/></span>
                        </li>
                        <li class="nav-item">
                        // <span class="nav-link active" aria-current="page" >{"Balance: "} <Balance/></span>
                        </li>
                    </ul>
                    // <form class="d-flex mt-3 mt-lg-0" role="search">
                    //     <input class="form-control me-2" type="search" placeholder="Search" aria-label="Search"/>
                    //     <button class="btn btn-outline-success" type="submit">{"Search"}</button>
                    // </form>
                    // <ul class="navbar-nav -flex mt-3 mt-lg-0">
                    //     <li class="nav-item">
                    //     <a class="nav-link active" aria-current="page" href="#"><AccountAddressView/></a>
                    //     </li>
                    // </ul>
                    </div>
                </div>
                </div>
            </nav>

        </>
    }
}
