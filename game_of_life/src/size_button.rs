use super::*;

pub struct Obj {
    width_ref:  NodeRef,
    height_ref: NodeRef,

    torus_ref: NodeRef,

    event_bus: Dispatcher<ToUniverse>,
}
pub enum Msg {
    ChangeSize,
}
impl Component for Obj {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { width_ref:  NodeRef::default(),
               height_ref: NodeRef::default(),
               torus_ref:  NodeRef::default(),
               event_bus:  ToUniverse::dispatcher(), }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use to_universe::In;
        use Msg::*;

        match msg {
            ChangeSize => {
                let width = self.width_ref
                                .cast::<HtmlInputElement>()
                                .unwrap()
                                .value()
                                .parse()
                                .unwrap();

                let height = self.height_ref
                                 .cast::<HtmlInputElement>()
                                 .unwrap()
                                 .value()
                                 .parse()
                                 .unwrap();

                let torus = self.torus_ref
                                .cast::<HtmlInputElement>()
                                .unwrap()
                                .checked();

                self.event_bus
                    .send(In::ChangeSize(width, height, torus));
                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let change_size = ctx.link().callback(|_| Msg::ChangeSize);

        html! {
            <div class={ "size-btn" }>
               <input type={ "number" } value={ 96 } ref={ self.width_ref.clone() } />
               <input type={ "number" } value={ 64 } ref={ self.height_ref.clone() } />
               <div>
                   <input type={ "checkbox" } ref={ self.torus_ref.clone() } checked={ true } />
                   <label>{ "Torus" }</label>
               </div>
               <button onclick={ change_size } >
                   { "Change Size" }
               </button>
            </div>
        }
    }
}
