use yew::prelude::*;
// use yew::html::*;
use web_sys::HtmlTextAreaElement;
// use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use rustservercube::*;
use std::collections::HashMap;
use gloo::console::log;

enum Msg {
    InpScram(String),
    Solve
}

struct Scrmm{
    // link: ComponentLink<Self>,
    scram:String,
    sol: String,
    to_dr_prune: HashMap<(u64,u64),[u8;8]>,
    to_finish_prune: HashMap<(u64,u64),[u8;8]>
}

impl Component for Scrmm{
    type Message =  Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self{
        Self {scram: "U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2".to_string(), 
        sol: String::from(""), 
        to_dr_prune: HashMap::new(),
        to_finish_prune: HashMap::new()}
    }

    fn update(&mut self,_ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InpScram(scram) => {
                self.scram = scram.clone();
                self.sol = "".to_string();
            }
            Msg::Solve => {
                if self.to_dr_prune.is_empty(){
                    self.to_dr_prune = gen_eo_to_dr_prune();
                    log!("dr prune generated");
                    self.to_finish_prune = gen_finish_prune();
                    log!("finish prune generated");
                }
                log!("solver started");
                self.sol = solve_eo_from_scrm(self.scram.clone(),&self.to_dr_prune,&self.to_finish_prune);
            }
            
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
                    // println!("attempt1");
                    link.callback(|_| Msg::Solve)}>{ "Solve" }</button>
                <p>{ format!("Solution: {}",self.sol)}</p>
            </div>

        }
    }
}

fn main() {
    yew::start_app::<Scrmm>();
}
