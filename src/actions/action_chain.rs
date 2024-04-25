use crate::action::{Action, Result};

pub struct ActionChain {
    last_action_index: Option<usize>,
    actions: Vec<Box<dyn Action>>,
}

impl ActionChain {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            last_action_index: None,
            actions: vec![],
        })
    }

    pub fn add(&mut self, action: Box<dyn Action>) -> &mut Self {
        self.actions.push(action);
        self
    }
}

impl Action for ActionChain {
    fn execute(&mut self) -> Result<()> {
        for (action_counter, action) in self.actions.iter_mut().enumerate() {
            self.last_action_index = Some(action_counter);

            action.execute()?;
        }

        Ok(())
    }

    fn undo(&mut self) -> Result<()> {
        match self.last_action_index {
            None => Ok(()),
            Some(index) => {
                for (undo_index, action) in self.actions[..=index].iter_mut().rev().enumerate() {
                    let res = action.undo();

                    if res.is_err() {
                        self.last_action_index = Some(index - undo_index);
                        return res;
                    }
                }

                self.last_action_index = None;
                Ok(())
            }
        }
    }
}
