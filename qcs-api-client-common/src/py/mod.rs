/// Implements __repr__ on a [`pyo3::pyclass`] by using it's [`std::fmt::Debug`]
/// implementation.
///
/// The pyclass must implement [`std::fmt::Debug`].
#[macro_export]
macro_rules! impl_repr {
    ($name: ident) => {
        #[pyo3::pymethods]
        impl $name {
            #[must_use]
            fn __repr__(&self) -> String {
                format!("{:?}", self)
            }
        }
    };
}

/// Implement `__str__` for wrapper types whose inner type implements [`Display`](std::fmt::Display).
#[macro_export]
macro_rules! impl_str {
    ($name: ident) => {
        #[$crate::pyo3::pymethods]
        impl $name {
            fn __str__(&self) -> String {
                format!("{}", $crate::PyWrapper::as_inner(self))
            }
        }
    };
}

/// Provides support for equality checks of a [`pyo3::pyclass`] by implementing `__richcmp__`.
///
/// The pyclass must implement [`PartialEq`].
#[macro_export]
macro_rules! impl_eq {
    ($name: ident) => {
        #[pymethods]
        impl $name {
            fn __richcmp__(
                &self,
                py: pyo3::Python<'_>,
                other: &Self,
                op: pyo3::pyclass::CompareOp,
            ) -> pyo3::PyObject {
                use pyo3::IntoPy;
                match op {
                    pyo3::pyclass::CompareOp::Eq => (self == other).into_py(py),
                    pyo3::pyclass::CompareOp::Ne => (self != other).into_py(py),
                    _ => py.NotImplemented(),
                }
            }
        }
    };
}
