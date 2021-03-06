use jni_sys::{JavaVM, JNIEnv};
use std::ptr;
use std::os::raw::c_void;

// =================================================================================================

/// A native thread's attachment to an embedded JVM.
/// The thread is automatically detached when `JvmAttachment` goes out of scope (RAII).
pub struct JvmAttachment {

    /// The JNI environment.
    jni_environment: *mut JNIEnv,

    /// The JVM.
    jvm: *mut JavaVM,
}


impl JvmAttachment {

    ///
    pub unsafe fn new(jvm: *mut JavaVM) -> JvmAttachment {

        // Initialize the data.
        let mut jvm_attachment = JvmAttachment {
            jni_environment: ptr::null_mut(),
            jvm: jvm,
        };

        // Try to attach the current native thread to an embedded JVM.
        let _ = (**jvm).AttachCurrentThread.unwrap()(
            jvm,
            (&mut jvm_attachment.jni_environment as *mut *mut JNIEnv) as *mut *mut c_void,
            ptr::null_mut(),
        );

        // TODO: interpret the result

        jvm_attachment
    }

    ///
    pub fn jni_environment(&self) -> *mut JNIEnv {
        self.jni_environment
    }
}

// =================================================================================================

impl Drop for JvmAttachment {

    fn drop(&mut self) {

        unsafe {
            // Try to detach the current native thread from the embedded JVM.
            let _ = (**self.jvm).DetachCurrentThread.unwrap()(
                self.jvm,
            );

            // TODO: interpret the result
        }
    }
}

// =================================================================================================