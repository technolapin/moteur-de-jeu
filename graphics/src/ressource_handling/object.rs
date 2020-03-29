use std::sync::Arc;

use super::Group;
use crate::engine::{ProgramId, Params};

    
/**
This structure represents a drawable 3D object.
It is basicaly a set of groups associated with shaders programs, and some rendering parameters.
It owns no heavy data, only contains atomic shared pointers.
 */
#[derive(Debug)]
pub struct Object
{
    pub data: Vec<(Group, ProgramId)>,
    pub params: Arc<Params>
}


impl Object
{
    pub fn new(v: Vec<(Group, ProgramId)>, params: Arc<Params>) -> Self
    {
        Self{data: v, params: params}
    }
}
