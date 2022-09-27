use super::*;

pub struct Obj {
    input_ref: NodeRef,
    event_bus: Dispatcher<ToUniverse>,
}
pub enum Msg {
    Generate,
    Clear,
}
impl Component for Obj {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let input_ref = NodeRef::default();
        let event_bus = ToUniverse::dispatcher();

        Self { event_bus, input_ref }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use to_universe::In;
        use Msg::*;

        match msg {
            Generate => {
                let prob = self.input_ref
                               .cast::<HtmlInputElement>()
                               .unwrap()
                               .value()
                               .parse()
                               .unwrap();

                self.event_bus.send(In::Generate(prob));
                false
            },

            Clear => {
                self.event_bus.send(In::Clear);
                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let generate = ctx.link().callback(|_| Msg::Generate);
        let clear = ctx.link().callback(|_| Msg::Clear);

        html! {
            <div class={ "gen-btn" }>
                <input type={ "range" } min={ 0 } max={ 100 } step={ 5 } ref={ self.input_ref.clone() } />
                <button onclick={ generate }>
                    { "Generate" }
                </button>
                <button onclick={ clear }>
                    { "Clear" }
                </button>
            </div>
        }
    }
}
