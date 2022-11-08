use yew::prelude::*;



#[function_component(App)]
fn app() -> Html {
	html! {
		<>
			<div class={ "option" }>
				<div class={ "config" }>
				</div>
			</div>
		</>
	}
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
