use super::*;

pub struct Obj {
    link:        AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
}
pub enum In {
    Play,
    Pause,
    Step,

	RunFor(u32),

    ChangeSpeed(u16),

    Generate(u8),
    Clear,

    ChangeSize(usize, usize, bool),

    ChangeRules(Rules),
}
impl Agent for Obj {
    type Input = In;
    type Message = ();
    type Output = Box<universe::Msg>;
    type Reach = yew_agent::Context<Self>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link,
               subscribers: HashSet::new() }
    }

    fn update(&mut self, msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: yew_agent::HandlerId) {
        use universe::Msg as Out;

        match msg {
            In::Play =>
                for sub in self.subscribers.iter() {
                    self.link
                        .respond(*sub, Box::new(Out::Play));
                },

            In::Pause =>
                for sub in self.subscribers.iter() {
                    self.link
                        .respond(*sub, Box::new(Out::Pause));
                },

            In::Step =>
                for sub in self.subscribers.iter() {
                    self.link
                        .respond(*sub, Box::new(Out::Step));
                },

			In::RunFor(value) =>
				for sub in self.subscribers.iter() {
					self.link
						.respond(*sub, Box::new(Out::RunFor(value)));
				},

            In::ChangeSpeed(speed) =>
                for sub in self.subscribers.iter() {
                    self.link
                        .respond(*sub, Box::new(Out::ChangeSpeed(speed)));
                },

            In::Generate(prob) =>
                for sub in self.subscribers.iter() {
                    self.link
                        .respond(*sub, Box::new(Out::Generate(prob)));
                },

            In::Clear =>
                for sub in self.subscribers.iter() {
                    self.link
                        .respond(*sub, Box::new(Out::Clear));
                },

            In::ChangeSize(width, height, torus) =>
                for sub in self.subscribers.iter() {
                    self.link
                        .respond(*sub, Box::new(Out::ChangeSize(width, height, torus)));
                },

            In::ChangeRules(rules) =>
                for sub in self.subscribers.iter() {
                    self.link
                        .respond(*sub, Box::new(Out::ChangeRules(rules.clone())));
                },
        }
    }

    fn connected(&mut self, id: yew_agent::HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: yew_agent::HandlerId) {
        self.subscribers.remove(&id);
    }
}
