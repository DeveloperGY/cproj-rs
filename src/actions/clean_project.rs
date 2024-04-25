use crate::{
    action::{Action, Result},
    ActionChain, ClearDirectory, CreateDirectory, HasFile,
};

pub struct CleanProject {
    action_chain: Box<ActionChain>,
}

impl CleanProject {
    pub fn new() -> Box<Self> {
        let mut action_chain = ActionChain::new();
        action_chain
            .add(HasFile::new("cproj.json"))
            .add(ClearDirectory::new("bin"))
            .add(CreateDirectory::new("bin"))
            .add(CreateDirectory::new("bin/debug"))
            .add(CreateDirectory::new("bin/release"))
            .add(CreateDirectory::new("bin/debug/log"))
            .add(CreateDirectory::new("bin/debug/obj"))
            .add(CreateDirectory::new("bin/release/log"))
            .add(CreateDirectory::new("bin/release/obj"));

        Box::new(Self { action_chain })
    }
}

impl Action for CleanProject {
    fn execute(&mut self) -> Result<()> {
        println!("=> Cleaning Project...");
        self.action_chain.execute()
    }

    fn undo(&mut self) -> Result<()> {
        self.action_chain.undo()
    }
}
