use crate::{InputValue, MainInputs, Parameter, ProgramInputs, ProgramState};
use leo_inputs::{
    files::{File, TableOrSection},
    InputParserError,
};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
pub struct Inputs {
    inputs: ProgramInputs,
    state: ProgramState,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            inputs: ProgramInputs::new(),
            state: ProgramState::new(),
        }
    }

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private inputs.
    pub fn empty(&self) -> Self {
        let inputs = self.inputs.empty();
        let state = self.state.empty();

        Self { inputs, state }
    }

    /// Returns the number of input variables to pass into the `main` program function
    pub fn len(&self) -> usize {
        self.inputs.len() + self.state.len()
    }

    /// Manually set the input variables to the `main` program function
    pub fn set_main_inputs(&mut self, inputs: MainInputs) {
        self.inputs.main = inputs;
    }

    /// Parse all inputs included in a file and store them in `self`.
    /// Currently parser does not care if file is `.in` or `.state`
    pub fn parse(&mut self, file: File) -> Result<(), InputParserError> {
        for entry in file.entries.into_iter() {
            match entry {
                TableOrSection::Section(section) => {
                    self.inputs.parse(section)?;
                }
                TableOrSection::Table(table) => {
                    self.state.parse(table)?;
                }
            }
        }

        Ok(())
    }

    /// Returns the main function input value with the given `name`
    pub fn get(&self, name: &String) -> Option<Option<InputValue>> {
        self.inputs.get(name)
    }

    /// Returns the runtime register input values
    pub fn get_registers(&self) -> HashMap<Parameter, Option<InputValue>> {
        self.inputs.get_registers()
    }

    /// Returns the runtime record input values
    pub fn get_record(&self) -> HashMap<Parameter, Option<InputValue>> {
        self.state.get_record()
    }

    /// Returns the runtime state input values
    pub fn get_state(&self) -> HashMap<Parameter, Option<InputValue>> {
        self.state.get_state()
    }

    /// Returns the runtime state leaf input values
    pub fn get_state_leaf(&self) -> HashMap<Parameter, Option<InputValue>> {
        self.state.get_state_leaf()
    }
}
