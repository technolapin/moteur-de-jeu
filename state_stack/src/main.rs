// Le jeu peut être dans un seul état
// Cette enum contient les états possibles
#[derive(Debug, Clone, Copy)]
pub enum States {
	MainMenu,
	Game,
	PauseMenu,
	Parametre,
	None, // Par défaut
}

pub struct CurrentState {
	// Il n'y a pas de pile en Rust, on utilise un vecteur
	stack_of_states : Vec<States>,
}

impl CurrentState {
    pub fn new() -> Self {
    	return Self{ stack_of_states : Vec::new() };
    }
    
    pub fn push_state( mut self, state : States ) {
    	self.stack_of_states.push( state );
    }
    
    pub fn get_current_state( self ) -> States {
    	return match self.stack_of_states.last() {
    		Some( state ) => *state,
    		_ => States::None
    	};
    }
    
    pub fn pop_state( mut self ) {
    	self.stack_of_states.pop();
    }
}


// TEST
pub fn main() {
	let _test : CurrentState;
}
