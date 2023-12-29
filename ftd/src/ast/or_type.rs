#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct OrType {
    pub name: String,
    pub variants: Vec<OrTypeVariant>,
    pub line_number: usize,
}

pub const ORTYPE: &str = "or-type";

impl OrType {
    fn new(name: &str, variants: Vec<ftd::ast::OrTypeVariant>, line_number: usize) -> OrType {
        OrType {
            name: name.to_string(),
            variants,
            line_number,
        }
    }

    pub(crate) fn is_or_type(section: &ftd::p1::Section) -> bool {
        section.kind.as_ref().map_or(false, |s| s.eq(ORTYPE))
    }

    pub(crate) fn from_p1(section: &ftd::p1::Section, doc_id: &str) -> ftd::ast::Result<OrType> {
        if !Self::is_or_type(section) {
            return ftd::ast::parse_error(
                format!("Section is not or-type section, found `{:?}`", section),
                doc_id,
                section.line_number,
            );
        }
        let mut variants = vec![];
        for section in section.sub_sections.iter() {
            variants.push(OrTypeVariant::from_p1(section, doc_id)?);
        }

        Ok(OrType::new(
            section.name.as_str(),
            variants,
            section.line_number,
        ))
    }

    pub fn line_number(&self) -> usize {
        self.line_number
    }
}

impl ftd::ast::Field {
    pub(crate) fn from_p1(
        section: &ftd::p1::Section,
        doc_id: &str,
    ) -> ftd::ast::Result<ftd::ast::Field> {
        if !ftd::ast::VariableDefinition::is_variable_definition(section) {
            return ftd::ast::parse_error(
                format!(
                    "Section is not or-type variant section, found `{:?}`",
                    section
                ),
                doc_id,
                section.line_number,
            );
        }

        let kind = ftd::ast::VariableKind::get_kind(
            section.kind.as_ref().unwrap().as_str(),
            doc_id,
            section.line_number,
        )?;

        let value = ftd::ast::VariableValue::from_p1_with_modifier(section, doc_id, &kind)?.inner();

        Ok(ftd::ast::Field::new(
            section.name.trim_start_matches(ftd::ast::utils::REFERENCE),
            kind,
            ftd::ast::utils::is_variable_mutable(section.name.as_str()),
            value,
            section.line_number,
            Default::default(),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum OrTypeVariant {
    AnonymousRecord(ftd::ast::Record),
    Regular(ftd::ast::Field),
    Constant(ftd::ast::Field),
}

impl OrTypeVariant {
    pub fn new_record(record: ftd::ast::Record) -> OrTypeVariant {
        OrTypeVariant::AnonymousRecord(record)
    }

    pub fn new_variant(variant: ftd::ast::Field) -> OrTypeVariant {
        OrTypeVariant::Regular(variant)
    }

    pub fn new_constant(variant: ftd::ast::Field) -> OrTypeVariant {
        OrTypeVariant::Constant(variant)
    }

    pub fn set_name(&mut self, name: &str) {
        let variant_name = match self {
            OrTypeVariant::AnonymousRecord(r) => &mut r.name,
            OrTypeVariant::Regular(f) => &mut f.name,
            OrTypeVariant::Constant(f) => &mut f.name,
        };
        *variant_name = name.to_string();
    }

    pub fn name(&self) -> String {
        match self {
            OrTypeVariant::AnonymousRecord(r) => r.name.to_string(),
            OrTypeVariant::Regular(f) => f.name.to_string(),
            OrTypeVariant::Constant(f) => f.name.to_string(),
        }
    }

    pub(crate) fn is_constant(section: &ftd::p1::Section) -> bool {
        section
            .name
            .starts_with(format!("{} ", ftd::ast::constants::CONSTANT).as_str())
    }

    pub fn from_p1(section: &ftd::p1::Section, doc_id: &str) -> ftd::ast::Result<OrTypeVariant> {
        if ftd::ast::Record::is_record(section) {
            Ok(OrTypeVariant::new_record(ftd::ast::Record::from_p1(
                section, doc_id,
            )?))
        } else if OrTypeVariant::is_constant(section) {
            let mut section = section.to_owned();
            section.name = section
                .name
                .trim_start_matches(ftd::ast::constants::CONSTANT)
                .trim()
                .to_string();
            Ok(OrTypeVariant::new_constant(ftd::ast::Field::from_p1(
                &section, doc_id,
            )?))
        } else {
            Ok(OrTypeVariant::new_constant(ftd::ast::Field::from_p1(
                section, doc_id,
            )?))
        }
    }
}
