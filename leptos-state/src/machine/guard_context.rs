//! Context-based guard implementations

use super::*;

/// Context field equality guard
pub struct FieldEqualityGuard<C, E, T, F> {
    /// Field extractor function
    pub field_extractor: F,
    /// Expected value to compare against
    pub expected_value: T,
    /// Description of the guard
    pub description: String,
    /// Phantom data for type parameters
    _phantom: std::marker::PhantomData<(C, E)>,
}

impl<C, E, T, F> FieldEqualityGuard<C, E, T, F>
where
    F: Fn(&C) -> &T + 'static,
    T: PartialEq + Clone + 'static,
{
    /// Create a new field equality guard
    pub fn new(field_extractor: F, expected_value: T) -> Self {
        Self {
            field_extractor,
            expected_value,
            description: "Field Equality Guard".to_string(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create a new field equality guard with description
    pub fn with_description(field_extractor: F, expected_value: T, description: String) -> Self {
        Self {
            field_extractor,
            expected_value,
            description,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<C: std::fmt::Debug + 'static, E: std::fmt::Debug + PartialEq + 'static, T, F> GuardEvaluator<C, E> for FieldEqualityGuard<C, E, T, F>
where
    F: Fn(&C) -> &T + Clone + 'static,
    T: PartialEq + Clone + 'static,
{
    fn check(&self, context: &C, _event: &E) -> bool {
        let field_value = (self.field_extractor)(context);
        field_value == &self.expected_value
    }

    fn description(&self) -> String {
        format!("{} (field == {:?})", self.description, self.expected_value)
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            field_extractor: self.field_extractor.clone(),
            expected_value: self.expected_value.clone(),
            description: self.description.clone(),
            _phantom: std::marker::PhantomData,
        })
    }
}

/// Range guard - checks if a numeric field is within a range
pub struct RangeGuard<C, E, T, F> {
    /// Field extractor function
    pub field_extractor: F,
    /// Minimum value (inclusive)
    pub min_value: Option<T>,
    /// Maximum value (inclusive)
    pub max_value: Option<T>,
    /// Description of the guard
    pub description: String,
    /// Phantom data for type parameters
    _phantom: std::marker::PhantomData<(C, E)>,
}

impl<C, E, T, F> RangeGuard<C, E, T, F>
where
    F: Fn(&C) -> &T + 'static,
    T: PartialOrd + Clone + 'static,
{
    /// Create a new range guard
    pub fn new(field_extractor: F) -> Self {
        Self {
            field_extractor,
            min_value: None,
            max_value: None,
            description: "Range Guard".to_string(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Set the minimum value
    pub fn min(mut self, min_value: T) -> Self {
        self.min_value = Some(min_value);
        self
    }

    /// Set the maximum value
    pub fn max(mut self, max_value: T) -> Self {
        self.max_value = Some(max_value);
        self
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C: std::fmt::Debug, E: std::fmt::Debug + PartialEq, T, F> GuardEvaluator<C, E> for RangeGuard<C, E, T, F>
where
    F: Fn(&C) -> &T + Clone + 'static,
    T: PartialOrd + Clone + 'static,
{
    fn check(&self, context: &C, _event: &E) -> bool {
        let field_value = (self.field_extractor)(context);

        let min_ok = self
            .min_value
            .as_ref()
            .map_or(true, |min| field_value >= min);

        let max_ok = self
            .max_value
            .as_ref()
            .map_or(true, |max| field_value <= max);

        min_ok && max_ok
    }

    fn description(&self) -> String {
        let min_str = self
            .min_value
            .as_ref()
            .map_or("unbounded".to_string(), |v| format!("{:?}", v));
        let max_str = self
            .max_value
            .as_ref()
            .map_or("unbounded".to_string(), |v| format!("{:?}", v));

        format!("{} ({} to {})", self.description, min_str, max_str)
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            field_extractor: self.field_extractor.clone(),
            min_value: self.min_value.clone(),
            max_value: self.max_value.clone(),
            description: self.description.clone(),
            _phantom: std::marker::PhantomData,
        })
    }
}

/// Comparison guard - compares two fields
pub struct ComparisonGuard<C, E, T, F1, F2> {
    /// First field extractor function
    pub field1_extractor: F1,
    /// Second field extractor function
    pub field2_extractor: F2,
    /// Comparison operation
    pub comparison: ComparisonOp,
    /// Description of the guard
    pub description: String,
    /// Phantom data for type parameters
    _phantom: std::marker::PhantomData<(C, E, T)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOp {
    /// Equal
    Equal,
    /// Not equal
    NotEqual,
    /// Greater than
    GreaterThan,
    /// Less than
    LessThan,
    /// Greater than or equal
    GreaterEqual,
    /// Less than or equal
    LessEqual,
}

impl<C, E, T, F1, F2> ComparisonGuard<C, E, T, F1, F2>
where
    F1: Fn(&C) -> &T + 'static,
    F2: Fn(&C) -> &T + 'static,
    T: PartialOrd + PartialEq + Clone + 'static,
{
    /// Create a new comparison guard
    pub fn new(field1_extractor: F1, field2_extractor: F2, comparison: ComparisonOp) -> Self {
        Self {
            field1_extractor,
            field2_extractor,
            comparison,
            description: "Comparison Guard".to_string(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C: std::fmt::Debug, E: std::fmt::Debug + PartialEq, T, F1, F2> GuardEvaluator<C, E> for ComparisonGuard<C, E, T, F1, F2>
where
    F1: Fn(&C) -> &T + Clone + 'static,
    F2: Fn(&C) -> &T + Clone + 'static,
    T: PartialOrd + PartialEq + Clone + 'static,
{
    fn check(&self, context: &C, _event: &E) -> bool {
        let value1 = (self.field1_extractor)(context);
        let value2 = (self.field2_extractor)(context);

        match self.comparison {
            ComparisonOp::Equal => value1 == value2,
            ComparisonOp::NotEqual => value1 != value2,
            ComparisonOp::GreaterThan => value1 > value2,
            ComparisonOp::LessThan => value1 < value2,
            ComparisonOp::GreaterEqual => value1 >= value2,
            ComparisonOp::LessEqual => value1 <= value2,
        }
    }

    fn description(&self) -> String {
        let op_str = match self.comparison {
            ComparisonOp::Equal => "==",
            ComparisonOp::NotEqual => "!=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::LessThan => "<",
            ComparisonOp::GreaterEqual => ">=",
            ComparisonOp::LessEqual => "<=",
        };

        format!("{} (field1 {} field2)", self.description, op_str)
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            field1_extractor: self.field1_extractor.clone(),
            field2_extractor: self.field2_extractor.clone(),
            comparison: self.comparison.clone(),
            description: self.description.clone(),
            _phantom: std::marker::PhantomData,
        })
    }
}

/// Null check guard - checks if a field is null/none
pub struct NullCheckGuard<C, E, F> {
    /// Field extractor function
    pub field_extractor: F,
    /// Whether to check for null (true) or not null (false)
    pub check_null: bool,
    /// Description of the guard
    pub description: String,
    /// Phantom data for type parameters
    _phantom: std::marker::PhantomData<(C, E)>,
}

impl<C, E, F, T> NullCheckGuard<C, E, F>
where
    F: Fn(&C) -> Option<&T> + 'static,
{
    /// Create a new null check guard
    pub fn new(field_extractor: F, check_null: bool) -> Self {
        Self {
            field_extractor,
            check_null,
            description: "Null Check Guard".to_string(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create a guard that checks for null values
    pub fn is_null(field_extractor: F) -> Self {
        Self::new(field_extractor, true)
    }

    /// Create a guard that checks for non-null values
    pub fn is_not_null(field_extractor: F) -> Self {
        Self::new(field_extractor, false)
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C: std::fmt::Debug, E: std::fmt::Debug + PartialEq, F, T> GuardEvaluator<C, E> for NullCheckGuard<C, E, F>
where
    F: Fn(&C) -> Option<&T> + Clone + 'static,
{
    fn check(&self, context: &C, _event: &E) -> bool {
        let field_value = (self.field_extractor)(context);
        let is_null = field_value.is_none();

        if self.check_null {
            is_null
        } else {
            !is_null
        }
    }

    fn description(&self) -> String {
        let null_str = if self.check_null { "null" } else { "not null" };
        format!("{} (field is {})", self.description, null_str)
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            field_extractor: self.field_extractor.clone(),
            check_null: self.check_null,
            description: self.description.clone(),
            _phantom: std::marker::PhantomData,
        })
    }
}
