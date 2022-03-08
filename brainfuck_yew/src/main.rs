use brainfuck_interpreter::interpret;
use input::Input;
use output::Output;
use textarea::TextArea;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod input;
mod output;
mod textarea;

enum Msg {
    UpdateSource(String),
    UpdateInput(String),
    Execute,
}

struct Model {
    source: String,
    input: String,
    output: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            source: String::new(),
            input: String::new(),
            output: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateSource(value) => {
                self.source = value;
                true
            }
            Msg::UpdateInput(value) => {
                self.input = value;
                true
            }
            Msg::Execute => {
                let output = {
                    let input = Box::new(self.input.as_bytes());
                    interpret(&self.source, input)
                };
                self.output = match output {
                    Ok(result) => result,
                    Err(err) => err.to_string(),
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <section class="hero">
                    <div class="hero-body">
                        <p class="title">
                        {"Brainfuck interpreter"}
                        </p>
                        <p class="subtitle">
                        {"Execute your source code and input"}
                        </p>
                    </div>
                </section>
                <section class="section">
                    <TextArea value={self.source.clone()} oninput={ctx.link().callback(|e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Msg::UpdateSource(input.value())
                    })}/>
                    <Input value={self.input.clone()} oninput={ctx.link().callback(|e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Msg::UpdateInput(input.value())
                    })}/>
                    <Output value={self.output.clone()} />
                    <div class="control">
                        <button class="button is-link" onclick={ctx.link().callback(|_| {
                            Msg::Execute
                        })}>{"Execute"}</button>
                    </div>
                </section>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
