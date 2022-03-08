use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OutputProps {
    pub value: String,
}

#[function_component(Output)]
pub fn output(props: &OutputProps) -> Html {
    html! {
        <div class="field">
            <label class="label">{"Output"}</label>
            <div class="control">
                <textarea class="textarea" placeholder="Program output" value={props.value.clone()} readonly={true}/>
            </div>
        </div>
    }
}
