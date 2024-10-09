use std::str::FromStr;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
enum Choice {
    #[default]
    Foo,
    Bar,
    Baz,
}

impl Choice {
    fn all() -> impl Iterator<Item = Self> {
        [Choice::Foo, Choice::Bar, Choice::Baz].into_iter()
    }
}

impl FromStr for Choice {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Foo" => Choice::Foo,
            "Bar" => Choice::Bar,
            "Baz" => Choice::Baz,
            _ => return Err(format!("invalid choice: `{s}`")),
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Msg {
    Reset,
    Edit(Choice),
}

#[derive(Clone)]
struct App {
    selected: Choice,
    debug: Vec<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { selected: Choice::default(), debug: Vec::new() }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.debug.push(format!("{:>4}: {msg:?}\n", self.debug.len()));
        match msg {
            Msg::Edit(choice) => self.selected = choice,
            Msg::Reset => self.selected = Choice::default(),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::Edit(input.value().parse().expect("parse choice from choice"))
        });
        let onclick = ctx.link().callback(|_| Msg::Reset);
        let style = "display: grid; grid-template-columns: auto; gap: 10px; max-width: 200px;";

        html!(
            <form {style}>
                <button type="button" {onclick}>{"Reset"}</button>
                <span>{format!("Selected: {:?}", self.selected)}</span>
                <select name="selection" {oninput} autocomplete="off">{
                    Choice::all().map(|choice| {
                        let selected = choice == self.selected;
                        html!(
                            <option name="selection" value={format!("{choice:?}")} {selected}>
                                {format!("{choice:?}")}
                            </option>
                        )
                    }).collect::<Html>()
                }</select>
                <pre>{"Messages:\n"}{self.debug.iter().cloned().rev().collect::<String>()}</pre>
            </form>
        )
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
