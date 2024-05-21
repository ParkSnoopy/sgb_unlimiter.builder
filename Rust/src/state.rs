use crate::printer::{ success, debug, warn };
use crate::config::{ SUSPEND_SHOULD };

pub struct SuspendState {
    total: u32,
    tried: u32,
    fail_gethandle: u32,
    fail_accessdenied: u32,
    fail_suspendprocess: u32,
    success_suspend: u32,
}

impl SuspendState {
    pub fn new() -> Self {
        SuspendState {
            total: 0,
            tried: 0,
            fail_gethandle: 0,
            fail_accessdenied: 0,
            fail_suspendprocess: 0,
            success_suspend: 0,
        }
    }

    fn tried_suspend(&mut self) {
        self.total += 1;
        self.tried += 1;
    }

    pub fn no_match(&mut self) {
        self.total += 1;
    }

    pub fn fail_get_handle(&mut self) {
        self.tried_suspend();
        self.fail_gethandle += 1;
    }

    pub fn fail_access_denied(&mut self) {
        self.tried_suspend();
        self.fail_accessdenied += 1;
    }

    pub fn fail_suspend_process(&mut self) {
        self.tried_suspend();
        self.fail_suspendprocess += 1;
    }

    pub fn success_suspend_process(&mut self) {
        self.tried_suspend();
        self.success_suspend += 1;
    }

    pub fn display(&self) {
        success( format!("Done! [ {} / {} ]", self.success_suspend, self.total).as_str(), Some(format!("( with {} attempts )", self.tried).as_str()) );
        debug( format!("
            \r  - GetHandle Failed : {}
            \r  - Access Denied    : {}
            \r  - Suspend Failed   : {}
        ", self.fail_gethandle, self.fail_accessdenied, self.fail_suspendprocess).as_str(), None );
        if self.success_suspend < SUSPEND_SHOULD {
            warn( format!("Only {} unique process handled, some process may not handled", self.success_suspend).as_str(), None );
        }
    }
}
