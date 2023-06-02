use std::{borrow::Cow, fmt::{self, Debug}};
use serde::{Serialize, Deserialize};
use indexmap::{IndexMap};
use cfg_if::cfg_if;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ValidationErrorsKind {
  Struct(Box<ValidationErrors>),
  List(Vec<ValidationErrors>),
  Field(Vec<ValidationError>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FieldErrors {
  // pub errors: IndexMap<String, Vec<ValidationError>>
  pub errors: Vec<ValidationError>
}

impl FieldErrors {
  pub fn field(&self, field_name: &str) -> Option<Vec<ValidationError>> {
    let errs = self.errors
      .clone()
      .into_iter()
      .filter(|err| err.location.starts_with(field_name))
      .collect::<Vec<_>>();
    (!errs.is_empty()).then_some(errs)
  }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ValidationErrors(IndexMap<String, ValidationErrorsKind>);

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ValidationError {
  pub code: Cow<'static, str>,
  pub message: Option<Cow<'static, str>>,
  pub location: Cow<'static, str>,
  pub path: Vec<String>
}

impl ValidationError {
  pub fn simple(message: String) -> Self {
    Self{ code: Cow::Borrowed(""), message: Some(Cow::Owned(message)), location: Cow::Borrowed(""), path: vec![] }
  }
}

impl fmt::Display for ValidationError {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(msg) = self.message.as_ref() {
      write!(fmt, "{} ({})", msg, self.location)
    } else {
      write!(fmt, "Validation error: {}", self.code)
    }
  }
}

cfg_if! {
  if #[cfg(feature = "ssr")] {
    impl ValidationErrors {
      pub fn errors(&self) -> &IndexMap<String, ValidationErrorsKind> {
        &self.0
      }

      pub fn parse(&self) -> FieldErrors {
        fn parse_struct(field_errors: &mut Vec<ValidationError>, errs: &ValidationErrors, mut path: Vec<String>, pretty_path: &str) -> Result<(), Box<dyn std::error::Error>> {
          let base_len = path.len();
          for (inner_path, err) in errs.errors() {
            path.push(inner_path.to_string());
            parse_inner(field_errors, err, path.clone(), &format!("{pretty_path}.{inner_path}"))?;
            path.truncate(base_len);
          }
          Ok(())
        }
    
        fn parse_inner(field_errors: &mut Vec<ValidationError>, errs: &ValidationErrorsKind, mut path: Vec<String>, pretty_path: &str) -> Result<(), Box<dyn std::error::Error>> {
          match errs {
            ValidationErrorsKind::Field(errs) => {
              let errs = errs.iter().map(|err| {
                ValidationError {
                  location: format!("/{}", path.join("/")).into(),
                  path: path.clone(),
                  ..err.clone()
                }
              }).collect::<Vec<_>>();

              field_errors.extend(errs);
            },
            ValidationErrorsKind::Struct(errs) => parse_struct(field_errors, errs, path, pretty_path)?,
            ValidationErrorsKind::List(errs) => {
              let base_len = path.len();
              for (idx, err) in errs.iter().enumerate() {
                path.push(idx.to_string());
                parse_struct(field_errors, err, path.clone(), &format!("{pretty_path}[{idx}]"))?;
                path.truncate(base_len);
              }
            }
          }
          Ok(())
        }
    
        let mut field_errors = vec![];
        for (path, err) in self.errors() {
          _ = parse_inner(&mut field_errors, err, vec![path.to_string()], path);
        }
        FieldErrors{ errors: field_errors }
      }
    }

    impl From<&validator::ValidationError> for ValidationError {
      fn from(error: &validator::ValidationError) -> Self {
        Self{ code: error.code.clone(), message: error.message.clone(), location: Cow::Borrowed(""), path: vec![] }
      }
    }

    impl From<&validator::ValidationErrorsKind> for ValidationErrorsKind {
      fn from(errors_kind: &validator::ValidationErrorsKind) -> Self {
        match errors_kind {
          validator::ValidationErrorsKind::Field(errs) => ValidationErrorsKind::Field(errs.iter().map(Into::<ValidationError>::into).collect()),
          validator::ValidationErrorsKind::Struct(errs) => ValidationErrorsKind::Struct(Box::new(Into::<ValidationErrors>::into(&**errs))),
          validator::ValidationErrorsKind::List(errs) => {
            let to_errors = errs
              .iter()
              .map(|(_, v)| Into::<ValidationErrors>::into(&**v))
              .collect::<Vec<_>>();
            ValidationErrorsKind::List(to_errors)
          }
        }
      }
    }

    impl From<&validator::ValidationErrors> for ValidationErrors {
      fn from(errors: &validator::ValidationErrors) -> Self {
        let to_errors = errors.errors()
          .iter()
          .map(|(k, v)| (k.to_string(), Into::<ValidationErrorsKind>::into(v)))
          .collect::<IndexMap<String, ValidationErrorsKind>>();
        Self(to_errors)
      }
    }
  }
}