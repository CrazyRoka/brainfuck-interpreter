use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextAreaProps {
    pub value: String,
    pub oninput: Callback<InputEvent>,
}

#[function_component(TextArea)]
pub fn textarea(props: &TextAreaProps) -> Html {
    html! {
        <div class="field">
            <label class="label">{"Source"}</label>
            <div class="control">
                <textarea class="textarea" placeholder="Program source" value={props.value.clone()} oninput={&props.oninput} />
            </div>
        </div>
    }
}
