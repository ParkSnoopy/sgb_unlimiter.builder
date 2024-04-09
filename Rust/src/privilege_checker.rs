use crate::localutils::printer::warn;


pub fn dee_check() {
    use deelevate::{Token, PrivilegeLevel};

    let token = Token::with_current_process().unwrap();
    match token.privilege_level().unwrap() {
        PrivilegeLevel::NotPrivileged      => { warn("Currently [L-0] : NotPrivileged", None) }
        PrivilegeLevel::Elevated           => { warn("Currently [L-1] : Elevated", None) }
        PrivilegeLevel::HighIntegrityAdmin => { warn("Currently [L-2] : HighIntegrityAdmin", None) }
    }
}

pub fn _plv_check() { // x86_64
    use privilege_level::{x86_64_privilege_level, x86_64PrivilegeLevel};

    match x86_64_privilege_level() {
        x86_64PrivilegeLevel::Ring0 => warn("Currently [R-0] : Kernel Mode", None),
        x86_64PrivilegeLevel::Ring1 => warn("Currently [R-1] : Critical Driver", None),
        x86_64PrivilegeLevel::Ring2 => warn("Currently [R-2] : User Driver", None),
        x86_64PrivilegeLevel::Ring3 => warn("Currently [R-3] : User Mode", None),
    }
}

pub fn plv_check() { // etc.
    use privilege_level::{privilege_level, PrivilegeLevel};

    match privilege_level() {
        PrivilegeLevel::Hypervisor => warn("Currently       : Hypervisor", None),
        PrivilegeLevel::Kernel     => warn("Currently       : Kernel", None),
        PrivilegeLevel::Driver     => warn("Currently       : Driver", None),
        PrivilegeLevel::User       => warn("Currently       : User", None),
    }
}