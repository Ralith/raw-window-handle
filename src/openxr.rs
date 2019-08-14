/// Raw window handle for [OpenXR](https://www.khronos.org/openxr).
///
/// ## Construction
/// ```
/// # use raw_window_handle::openxr::OpenXrHandle;
/// let handle = OpenXrHandle {
///     /* fields */
///     ..OpenXrHandle::empty()
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenXrHandle {
    pub instance: u64,
    #[doc(hidden)]
    #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
    pub _non_exhaustive_do_not_use: crate::seal::Seal,
}

impl OpenXrHandle {
    pub fn empty() -> Self {
        #[allow(deprecated)]
        OpenXrHandle {
            instance: 0,
            _non_exhaustive_do_not_use: crate::seal::Seal,
        }
    }
}
