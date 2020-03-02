// Le jeu peut être dans un seul état
// Cette enum contient les états possibles
#[derive(Debug, Clone, Copy)]
pub enum States {
	MainMenu,
	Game,
	PauseMenu,
	Parametre,
	EtatDeBase, // Etat forcément en bas
	None // Par défaut, n'est utilisé que s'il n'y a pas d'état courant
}

pub enum StateContainer {
	ActiveState ( States ),
	InactiveState ( States )
}

pub struct CurrentState {
	// Il n'y a pas de pile en Rust, on utilise un vecteur
	stack_of_states : Vec<StateContainer>
}

impl CurrentState {
    pub fn new() -> Self {
    	return Self{ stack_of_states : vec![ StateContainer::ActiveState( States::EtatDeBase ) ] };
    }
    
    // Note : Travailler par référence, donc mettre &
    // Ca ne change rien au code dans la fonction, ni à l'utilisation externe
    pub fn push_state( &mut self, state : States ) {
    	self.stack_of_states.push( StateContainer::ActiveState( state ) );
    }
    
    pub fn get_current_state( &self ) -> States {
    	return match self.stack_of_states.last() {
    		Some( state_container ) => match state_container {
    			StateContainer::ActiveState ( state ) | StateContainer::InactiveState( state ) => *state
			},
			_ => States::None // Ne devrait jamais arriver
    	};
    }
    
    pub fn is_current_state_active( &self ) -> bool {
    	return match self.stack_of_states.last() {
    		Some( state_container ) => match state_container {
    			StateContainer::ActiveState ( _state ) => true,
    			StateContainer::InactiveState ( _state ) => false
    		},
    		_ => false // Ne devrait jamais arriver
    	};
    }
    
    pub fn pop_state( &mut self ) {
    	if self.stack_of_states.len() > 1 { // Il faut que EtatDeBase reste en bas
    		if !self.is_current_state_active() {
    			self.stack_of_states.pop();
    		}
    	}
    }
    
    pub fn set_current_state_inactive ( &mut self ) {
    	let current_state = self.get_current_state();
    	self.stack_of_states.pop();
    	self.stack_of_states.push( StateContainer::InactiveState( current_state ) );
    }
}


// TEST
pub fn main() {
	let _test : CurrentState;
}
