#[link(name = "lmath",
       vers = "0.1.0",
       uuid = "A2DEEB53-EB35-4B44-B247-9044B57E3BA5",
       author = "Brendan Zabarauskas",
       url = "https://github.com/bjz/lmath-rs")];

#[comment = "A generic linear algebra library."];
#[license = "ASL2"];
#[crate_type = "lib"];

extern mod std;

pub mod num;

pub mod mat;
pub mod quat;
pub mod vec;

pub mod projection;

#[test]
mod test {
    #[path = "test_mat.rs" ] mod mat;
    #[path = "test_quat.rs"] mod quat;
    #[path = "test_vec.rs" ] mod vec;
}
