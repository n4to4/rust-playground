struct Role {
    name: &'static str,
    disabled: bool,
    flag: u32,
}

fn main() {
    let role = unsafe {
        let mut role: Role = std::mem::zeroed();
        role.name = "basic";
        role.flag = 1;
        role.disabled = false;
        role
    };

    println!("{} ({} {})", role.name, role.flag, role.disabled);
}
