//! Feature flags for RV Extensions
//!
//! <https://community.bistudio.com/wiki/Extensions#Feature_Flags>

/// RVExtensionContext takes const void** as argument, instead of the default const char**, and arguments will be passed in their custom types
pub const RV_CONTEXT_ARGUMENTS_VOID_PTR: u64 = 1 << 0;
/// RVExtensionContext will retrieve a full Stacktrace
pub const RV_CONTEXT_STACK_TRACE: u64 = 1 << 1;
/// RVExtensionContext will not be called automatically. It must be manually requested via RVExtensionRequestContext (This improves performance when context is not needed).
pub const RV_CONTEXT_NO_DEFAULT_CALL: u64 = 1 << 2;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Feature flags for RV Extensions
pub struct FeatureFlags {
    context_stack_trace: bool,
}

impl FeatureFlags {
    /// Set the context_stack_trace flag
    pub fn set_context_stack_trace(&mut self, value: bool) {
        self.context_stack_trace = value;
    }

    pub fn with_context_stack_trace(mut self, value: bool) -> Self {
        self.set_context_stack_trace(value);
        self
    }

    /// Get the context_stack_trace flag
    pub fn context_stack_trace(&self) -> bool {
        self.context_stack_trace
    }

    /// Create a new FeatureFlags from the given bits
    pub fn from_bits(bits: u64) -> Self {
        let mut flags = Self::default();
        flags.set_context_stack_trace(bits & RV_CONTEXT_STACK_TRACE != 0);
        flags
    }

    /// Get the bits of the FeatureFlags
    pub fn as_bits(&self) -> u64 {
        let mut bits = RV_CONTEXT_NO_DEFAULT_CALL | RV_CONTEXT_ARGUMENTS_VOID_PTR;
        if self.context_stack_trace() {
            bits |= RV_CONTEXT_STACK_TRACE;
        }
        bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let flags = FeatureFlags::default();
        assert_eq!(
            flags.as_bits(),
            RV_CONTEXT_NO_DEFAULT_CALL | RV_CONTEXT_ARGUMENTS_VOID_PTR
        );
    }

    #[test]
    fn just_stack_trace() {
        let mut flags = FeatureFlags::default();
        flags.set_context_stack_trace(true);
        assert_eq!(
            flags.as_bits(),
            RV_CONTEXT_NO_DEFAULT_CALL | RV_CONTEXT_STACK_TRACE | RV_CONTEXT_ARGUMENTS_VOID_PTR
        );
    }
}
