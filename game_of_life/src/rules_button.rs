use super::*;

pub struct Obj {
    input_ref: NodeRef,
    event_bus: Dispatcher<ToUniverse>,
}
pub enum Msg {
    ChangeRules,
}
impl Component for Obj {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { input_ref: NodeRef::default(),
               event_bus: ToUniverse::dispatcher(), }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use to_universe::In;
        use Msg::*;

        match msg {
            ChangeRules => {
                let rules = self.input_ref
                                .cast::<HtmlInputElement>()
                                .unwrap()
                                .value();

                if let Ok(rules) = Rules::from_string(&rules) {
                    self.event_bus
                        .send(In::ChangeRules(rules));
                }

                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let change_rules = ctx.link()
                              .callback(|_| Msg::ChangeRules);

        html! {
            <div class={ "rules-btn" }>
                <input type={ "text" } value={ "B3/S23" } ref={ self.input_ref.clone() } />
                <button onclick={ change_rules }>
                    { "Change Rules" }
                </button>
            </div>
        }
    }
}
