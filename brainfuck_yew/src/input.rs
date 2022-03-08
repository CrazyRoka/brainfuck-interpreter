use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub value: String,
    pub oninput: Callback<InputEvent>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <div class="field">
            <label class="label">{"Source"}</label>
            <div class="control">
                <input type="text" class="input" placeholder="Program input" value={props.value.clone()} oninput={&props.oninput}/>
            </div>
        </div>
    }
}
