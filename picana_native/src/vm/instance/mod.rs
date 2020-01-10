//TODO: Vm utilities

pub mod exception {
    use dart_sys::{
        Dart_Handle, Dart_IsApiError, Dart_IsCompilationError, Dart_IsFatalError,
        Dart_IsUnhandledExceptionError,
    };
    use std::{error::Error, fmt};

    #[derive(Debug)]
    pub struct VmError {
        pub error: VmErrorType,
        pub handle: Dart_Handle,
    }

    impl VmError {
        pub unsafe fn from_handle(handle: Dart_Handle) -> Self {
            if Dart_IsApiError(handle) {
                return VmError {
                    error: VmErrorType::VmApiError,
                    handle,
                };
            } else if Dart_IsUnhandledExceptionError(handle) {
                return VmError {
                    error: VmErrorType::VmUnhandledException,
                    handle,
                };
            } else if Dart_IsCompilationError(handle) {
                return VmError {
                    error: VmErrorType::VmCompilationError,
                    handle,
                };
            } else if Dart_IsFatalError(handle) {
                return VmError {
                    error: VmErrorType::VmFatalError,
                    handle,
                };
            } else {
                VmError {
                    error: VmErrorType::VmNullPointer,
                    handle,
                }
            }
        }
    }

    impl Error for VmError {
        fn description(&self) -> &str {
            match self.error {
                VmErrorType::VmApiError => "API Error",
                VmErrorType::VmUnhandledException => "Unhandled Exception",
                VmErrorType::VmCompilationError => "Compilation Error",
                VmErrorType::VmFatalError => "Fatal Error",
                _ => "NUll Error?",
            }
        }
        fn cause(&self) -> Option<&Error> {
            None
        }
        fn source(&self) -> Option<&(Error + 'static)> {
            None
        }
    }

    impl fmt::Display for VmError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "VM Error => {}", self.description())
        }
    }

    #[derive(Debug)]
    pub enum VmErrorType {
        VmApiError,
        VmUnhandledException,
        VmCompilationError,
        VmFatalError,
        VmNullPointer,
    }
}
