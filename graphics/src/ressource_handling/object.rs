use super::Group;
use crate::engine::{ProgramId, Params};

use std::sync::Arc;
    
/**
This structure represents a 3D object.
It is created by the ModelsHolder, which owns the data.
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
