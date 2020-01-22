use crate::{DataSourceFieldRef, DomainError, Field, PrismaValue};

// Collection of fields that uniquely identify a record of a model.
// There can be different sets of fields at the same time identifying a model.
#[derive(Debug, Clone, Default)]
pub struct ModelIdentifier {
    fields: Vec<Field>,
}

impl From<Field> for ModelIdentifier {
    fn from(f: Field) -> Self {
        Self { fields: vec![f] }
    }
}

impl ModelIdentifier {
    pub fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }

    pub fn names<'a>(&'a self) -> impl Iterator<Item = &'a str> + 'a {
        self.fields.iter().map(|field| field.name())
    }

    pub fn fields<'a>(&'a self) -> impl Iterator<Item = &'a Field> + 'a {
        self.fields.iter()
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn is_singular_field(&self) -> bool {
        self.len() == 1
    }

    pub fn get(&self, name: &str) -> Option<&Field> {
        self.fields().find(|field| field.name() == name)
    }
}

impl IntoIterator for ModelIdentifier {
    type Item = Field;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

// Collection of field to value pairs corresponding to a ModelIdentifier the record belongs to.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordIdentifier {
    pub pairs: Vec<(DataSourceFieldRef, PrismaValue)>,
}

impl RecordIdentifier {
    pub fn new(pairs: Vec<(DataSourceFieldRef, PrismaValue)>) -> Self {
        Self { pairs }
    }

    pub fn add(&mut self, pair: (DataSourceFieldRef, PrismaValue)) {
        self.pairs.push(pair);
    }

    pub fn values(&self) -> impl Iterator<Item = PrismaValue> + '_ {
        self.pairs.iter().map(|p| p.1.clone())
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn misses_autogen_value(&self) -> bool {
        self.pairs.iter().any(|p| p.1.is_null())
    }

    pub fn add_autogen_value<V>(&mut self, value: V) -> bool
    where
        V: Into<PrismaValue>,
    {
        for pair in self.pairs.iter_mut() {
            if pair.1.is_null() {
                pair.1 = value.into();
                return true;
            }
        }

        return false;
    }
}

impl IntoIterator for RecordIdentifier {
    type Item = (DataSourceFieldRef, PrismaValue);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.pairs.into_iter()
    }
}

impl From<(DataSourceFieldRef, PrismaValue)> for RecordIdentifier {
    fn from(tup: (DataSourceFieldRef, PrismaValue)) -> Self {
        Self::new(vec![tup])
    }
}

impl From<Vec<(DataSourceFieldRef, PrismaValue)>> for RecordIdentifier {
    fn from(tup: Vec<(DataSourceFieldRef, PrismaValue)>) -> Self {
        Self::new(tup)
    }
}

#[derive(Debug, Clone)]
pub struct SingleRecord {
    pub record: Record,
    pub field_names: Vec<String>,
}

impl Into<ManyRecords> for SingleRecord {
    fn into(self) -> ManyRecords {
        ManyRecords {
            records: vec![self.record],
            field_names: self.field_names,
        }
    }
}

impl SingleRecord {
    pub fn new(record: Record, field_names: Vec<String>) -> Self {
        Self { record, field_names }
    }

    pub fn identifier(&self, id: &ModelIdentifier) -> crate::Result<RecordIdentifier> {
        self.record.identifier(&self.field_names, id)
    }

    pub fn get_field_value(&self, field: &str) -> crate::Result<&PrismaValue> {
        self.record.get_field_value(&self.field_names, field)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ManyRecords {
    pub records: Vec<Record>,
    pub field_names: Vec<String>,
}

impl ManyRecords {
    pub fn identifiers(&self, model_id: &ModelIdentifier) -> crate::Result<Vec<RecordIdentifier>> {
        self.records
            .iter()
            .map(|record| record.identifier(&self.field_names, model_id).map(|i| i.clone()))
            .collect()
    }

    /// Maps into a Vector of (field_name, value) tuples
    pub fn as_pairs(&self) -> Vec<Vec<(String, PrismaValue)>> {
        self.records
            .iter()
            .map(|record| {
                record
                    .values
                    .iter()
                    .zip(self.field_names.iter())
                    .map(|(value, name)| (name.clone(), value.clone()))
                    .collect()
            })
            .collect()
    }

    /// Reverses the wrapped records in place
    pub fn reverse(&mut self) {
        self.records.reverse();
    }
}

#[derive(Debug, Default, Clone)]
pub struct Record {
    pub values: Vec<PrismaValue>,
    pub parent_id: Option<RecordIdentifier>,
}

impl Record {
    pub fn new(values: Vec<PrismaValue>) -> Record {
        Record {
            values,
            ..Default::default()
        }
    }

    // Todo - Q: Are the `field_names` basically the column names of the underlying DB?
    pub fn identifier(&self, field_names: &[String], id: &ModelIdentifier) -> crate::Result<RecordIdentifier> {
        let pairs: Vec<(DataSourceFieldRef, PrismaValue)> = id
            .fields()
            .into_iter()
            .flat_map(|id_field| {
                let source_fields = match id_field {
                    Field::Scalar(sf) => vec![sf.data_source_field.clone()],
                    Field::Relation(rf) => rf.data_source_fields.clone(),
                };

                source_fields.into_iter().map(|source_field| {
                    self.get_field_value(field_names, &source_field.name)
                        .map(|val| (source_field, val.clone()))
                })
            })
            .collect::<crate::Result<Vec<_>>>()?;

        Ok(RecordIdentifier { pairs })
    }

    pub fn get_field_value(&self, field_names: &[String], field: &str) -> crate::Result<&PrismaValue> {
        let index = field_names.iter().position(|r| r == field).map(Ok).unwrap_or_else(|| {
            Err(DomainError::FieldNotFound {
                name: field.to_string(),
                model: String::new(),
            })
        })?;

        Ok(&self.values[index])
    }

    pub fn set_parent_id(&mut self, parent_id: RecordIdentifier) {
        self.parent_id = Some(parent_id);
    }
}
