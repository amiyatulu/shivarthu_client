use magic_crypt::generic_array::functional;
use yew::prelude::*;


#[function_component(AddProfile)]
pub fn add_profile() -> Html {
    html!{
        <>
        <div>
         <form>
            <div class="mb-3">
                <label for="seed" class="form-label">{"Seed"}</label>
                <input type="text" class="form-control" name="seed" />
            </div>
         </form>
        </div>
        </>
    }
}