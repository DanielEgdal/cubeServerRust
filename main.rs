use yew::prelude::*;
use yew::html::*;
use web_sys::HtmlTextAreaElement;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use rustservercube::solve_eo_from_scrm;

enum Msg {
    InpScram(String),
    Solve
}

struct Scrmm{
    // link: ComponentLink<Self>,
    scram:String,
    sol: Vec<u8>
}

impl Component for Scrmm{
    type Message =  Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self{
        Self {scram: "U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2".to_string(), sol: Vec::new()}
    }

    fn update(&mut self,_ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InpScram(scram) => self.scram = scram.clone(),
            Msg::Solve => self.sol = solve_eo_from_scrm(self.scram.clone()),
        }
        true
    }

    fn view(&self,_ctx: &Context<Self>) -> Html {
        let link = _ctx.link();
        html! {

            <div class="container">
                <p>{ self.scram.clone() }</p>
                <textarea
                    oninput={link.callback(|event:InputEvent| Msg::InpScram(event.target().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap().value()))}>
                    // oninput={link.callback(|event: InputData| Msg::InpScram(event))}>
                </textarea>
                <button onclick={
                    println!("attempt1");
                    link.callback(|_| Msg::Solve)}>{ "Solve" }</button>
                <p>{ format!("{:?}",self.sol)}</p>
            </div>

        }
    }
}

fn main() {
    yew::start_app::<Scrmm>();
}