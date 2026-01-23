pub struct Simulation {
    size: Size,
    state: Vec<Element>,
    // Indexes into the state vector.
    state_changes: Vec<StateChange>,
}

#[derive(Default, Clone, PartialEq)]
pub enum Element {
    Sand,
    #[default]
    Air,
}
pub struct Size {
    pub width: usize,
    pub height: usize,
}

pub struct StateChange {
    // Indexes into the state vector.
    pub state_index: usize,
    pub new_element: Element,
}

impl Simulation {
    pub fn new(size: Size) -> Self {
        Self {
            state: vec![Element::Air; size.width * size.height],
            size: size,
            state_changes: Vec::new(),
        }
    }

    // Tick the simulation

    // Update the state to include new state changes
    // Return a vector of indices into state that changed in the process step.
    pub fn tick(&mut self) {
        let mut new_state_changes = Vec::<StateChange>::new();

        // Loop over list of changes to create a new list of changes for next tick
        for state_change in &self.state_changes {
            // If state changes in this step again then push to new changes
            match state_change.new_element {
                Element::Air => {
                    let index_above = state_change.state_index.checked_sub(self.size.width);

                    if let Some(index_above) = index_above {
                        match self.state[index_above] {
                            Element::Sand => {
                                new_state_changes.push(StateChange {
                                    state_index: index_above,
                                    new_element: Element::Air,
                                });
                                new_state_changes.push(StateChange {
                                    state_index: state_change.state_index,
                                    new_element: Element::Sand,
                                });
                            }
                            Element::Air => (),
                        }
                    }
                }
                Element::Sand => {
                    let index_below = state_change.state_index + self.size.width;

                    match self.state.get(index_below) {
                        Some(&Element::Air) => {
                            new_state_changes.push(StateChange {
                                state_index: state_change.state_index,
                                new_element: Element::Air,
                            });
                            new_state_changes.push(StateChange {
                                state_index: index_below,
                                new_element: (Element::Sand),
                            });
                        }
                        // Do nonthing for now, in the future, go to the left or right
                        Some(&Element::Sand) => (),
                        // Out of bounds
                        None => (),
                    }
                }
            }

            // Apply queued changes in this tick
            self.state[state_change.state_index] = state_change.new_element.clone()
        }

        self.state_changes = new_state_changes;
    }

    // Modify simulation state
    pub fn add_sand(&mut self, x: usize, y: usize) {
        let index = y * self.size.width + x;

        if self.state[index] != Element::Sand {
            self.state_changes.push(StateChange {
                state_index: index,
                new_element: Element::Sand,
            });
        }
    }

    pub fn add_air(&mut self, x: usize, y: usize) {
        let index = y * self.size.width + x;

        if self.state[index] != Element::Air {
            self.state_changes.push(StateChange {
                state_index: index,
                new_element: Element::Air,
            });
        }
    }

    // Accessors
    pub fn get_size(&self) -> &Size {
        &self.size
    }

    pub fn get_state_changes(&self) -> &Vec<StateChange> {
        &self.state_changes
    }

    pub fn index_to_x_and_y(&self, index: usize) -> (usize, usize) {
        let x = index % self.size.width;
        let y = index / self.size.width;
        (x, y)
    }
}
