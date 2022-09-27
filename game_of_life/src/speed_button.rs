use super::*;

pub struct Obj {
    input_ref: NodeRef,
    event_bus: Dispatcher<ToUniverse>,
}
pub enum Msg {
    ChangeSpeed,
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
            ChangeSpeed => {
                let speed = self.input_ref
                                .cast::<HtmlInputElement>()
                                .unwrap()
                                .value()
                                .parse()
                                .unwrap();

                self.event_bus
                    .send(In::ChangeSpeed(speed));

                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let change_speed = ctx.link()
                              .callback(|_| Msg::ChangeSpeed);

        html! {
            <div class={ "speed-btn" }>
                <input type={ "range" } min={ 50 } max={ 1000 } step={ 50 } value={ 200 }  ref={ self.input_ref.clone() } />
                <button onclick={ change_speed }>
                    { "Change Speed" }
                </button>
            </div>
        }
    }
}
