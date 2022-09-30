use super::*;

pub struct Obj {
    state: State,

    input_ref: NodeRef,
    event_bus: Dispatcher<ToUniverse>,
}
#[derive(PartialEq)]
enum State {
    Playing,
    Paused,
}
pub enum Msg {
    Play,
    Pause,
    Step,

    RunFor,
}
impl Component for Obj {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { state: State::Paused,

               input_ref: NodeRef::default(),
               event_bus: ToUniverse::dispatcher(), }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use to_universe::In;
        use Msg::*;

        match msg {
            Play => {
                self.state = State::Playing;
                self.event_bus.send(In::Play);
                true
            },

            Pause => {
                self.state = State::Paused;
                self.event_bus.send(In::Pause);
                true
            },

            Step => {
                self.event_bus.send(In::Step);
                false
            },

            RunFor => {
                let value = self.input_ref
                                .cast::<HtmlInputElement>()
                                .unwrap()
                                .value()
                                .parse::<u32>()
                                .unwrap();

                self.event_bus.send(In::RunFor(value));
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let play = ctx.link().callback(|_| Msg::Play);
        let pause = ctx.link().callback(|_| Msg::Pause);
        let step = ctx.link().callback(|_| Msg::Step);
        let run_for = ctx.link().callback(|_| Msg::RunFor);

        html! {
            <>
                <div class={ "play-btn" }>
                    <button onclick={ play } disabled={ self.state == State::Playing }>
                        { "Play" }
                    </button>
                    <button onclick={ pause } disabled={ self.state == State::Paused }>
                        { "Pause" }
                    </button>
                    <button onclick={ step } disabled={ self.state == State::Playing }>
                        { "Step" }
                    </button>
                </div>
                <div class={ "run-btn" }>
                    <input type={ "number" } ref={ self.input_ref.clone() } disabled={ self.state == State::Playing }/>
                    <button onclick={ run_for } disabled={ self.state == State::Playing }>
                        { "Run for" }
                    </button>
                </div>
            </>
        }
    }
}
