mod relation;
mod scalar;

pub use relation::*;
pub use scalar::*;

use crate::prelude::*;
use datamodel::ScalarType;
use once_cell::sync::OnceCell;
use std::sync::Arc;

pub type DataSourceFieldRef = Arc<DataSourceField>;

#[derive(Debug)]
pub enum FieldTemplate {
    Relation(RelationFieldTemplate),
    Scalar(ScalarFieldTemplate),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Field {
    Relation(RelationFieldRef),
    Scalar(ScalarFieldRef),
}

#[derive(Debug, Clone)]
pub enum FieldWeak {
    Relation(RelationFieldWeak),
    Scalar(ScalarFieldWeak),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TypeIdentifier {
    String,
    Float,
    Boolean,
    Enum,
    Json,
    DateTime,
    GraphQLID,
    UUID,
    Int,
    Relation,
}

impl Field {
    pub fn name(&self) -> &str {
        match self {
            Field::Scalar(ref sf) => &sf.name,
            Field::Relation(ref rf) => &rf.name,
        }
    }

    pub fn is_scalar(&self) -> bool {
        match self {
            Field::Scalar(_) => true,
            Field::Relation(_) => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            Field::Scalar(ref sf) => sf.is_list,
            Field::Relation(ref rf) => rf.is_list,
        }
    }

    pub fn is_required(&self) -> bool {
        match self {
            Field::Scalar(ref sf) => sf.is_required,
            Field::Relation(ref rf) => rf.is_required,
        }
    }
}

impl FieldTemplate {
    pub fn build(self, model: ModelWeakRef) -> Field {
        match self {
            FieldTemplate::Scalar(st) => {
                let scalar = ScalarField {
                    name: st.name,
                    type_identifier: st.type_identifier,
                    is_required: st.is_required,
                    is_list: st.is_list,
                    is_auto_generated_int_id: st.is_auto_generated_int_id,
                    is_unique: st.is_unique,
                    internal_enum: st.internal_enum,
                    behaviour: st.behaviour,
                    model,
                    data_source_field: Arc::new(st.data_source_field),
                };

                Field::Scalar(Arc::new(scalar))
            }
            FieldTemplate::Relation(rt) => {
                let relation = RelationField {
                    name: rt.name,
                    is_required: rt.is_required,
                    is_list: rt.is_list,
                    is_auto_generated_int_id: rt.is_auto_generated_int_id,
                    is_unique: rt.is_unique,
                    relation_name: rt.relation_name,
                    relation_side: rt.relation_side,
                    model,
                    relation: OnceCell::new(),
                    data_source_fields: rt.data_source_fields.into_iter().map(Arc::new).collect(),
                };

                Field::Relation(Arc::new(relation))
            }
        }
    }
}

impl From<ScalarFieldRef> for Field {
    fn from(sf: ScalarFieldRef) -> Self {
        Field::Scalar(sf)
    }
}

impl From<RelationFieldRef> for Field {
    fn from(rf: RelationFieldRef) -> Self {
        Field::Relation(rf)
    }
}

impl From<ScalarType> for TypeIdentifier {
    fn from(st: ScalarType) -> Self {
        match st {
            ScalarType::String => Self::String,
            ScalarType::Int => Self::Int,
            ScalarType::Float => Self::Float,
            ScalarType::Boolean => Self::Boolean,
            ScalarType::Decimal => Self::Float,
            ScalarType::DateTime => Self::DateTime,
        }
    }
}
