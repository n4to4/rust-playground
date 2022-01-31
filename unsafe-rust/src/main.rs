use std::mem::MaybeUninit;
use std::ptr::addr_of_mut;

struct Role {
    name: &'static str,
    disabled: bool,
    flag: u32,
}

fn main() {
    let mut role = MaybeUninit::<Role>::uninit();
    let ptr = role.as_mut_ptr();

    // https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#initializing-a-struct-field-by-field
    let role = unsafe {
        addr_of_mut!((*ptr).name).write("maybe-uninit");
        addr_of_mut!((*ptr).disabled).write(false);
        addr_of_mut!((*ptr).flag).write(1);

        role.assume_init()
    };

    println!("{} ({} {})", role.name, role.flag, role.disabled);
}
