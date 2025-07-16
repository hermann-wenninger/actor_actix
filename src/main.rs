mod doit;
use doit::structs::done::Done;
use doit::structs::pending::Pending;

fn main()  {
let done: Done = Done::new("programming");
 println!("{}", done.super_struct.title);
 println!("{}", done.super_struct.status);
 let pending: Pending = Pending::new("teaching");
 println!("{}", pending.super_struct.title);
 println!("{}", pending.super_struct.status);
}
