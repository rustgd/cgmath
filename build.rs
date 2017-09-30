use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
use std::string::String;

/// Generate the name of the swizzle function and what it returns.
/// NOTE: This function assumes that variables are in ASCII format
fn gen_swizzle_nth(variables: &'static str, mut i: usize) -> (String, String) {
    let mut swizzle_impl = String::new();
    let mut swizzle = String::new();
    let n = variables.len();
    for _ in 0..n {
        let c = variables.as_bytes()[i%n] as char;
        swizzle.push(c);
        swizzle_impl.push_str(&format!("self.{}, ", c));
        i = i/n;
    }
    (swizzle, swizzle_impl)
}

/// This script generates the swizzle operators that are included in macros.rs
/// NOTE: This function assumes that variables are in ASCII format
fn gen_swizzle_functions(variables: &'static str) -> String {
    let n = variables.len();
    let vec_type = format!("$vector_type{}", n);
    let mut result = String::new();
    let nn = n.pow(n as u32);
    for i in 0..nn {
        let (swizzle_name, swizzle_impl) = gen_swizzle_nth(variables, i);
        result.push_str(
            &format!("        #[inline] pub fn {0}(&self) -> {2}<$S> {{ {2}::new({1}) }}\n",
            swizzle_name, swizzle_impl, vec_type));
    }
    result
}

fn main() {
    // The following scripts generates a macro for building swizzle operators for multidimensional
    // vectors or points.
    let out_dir = env::var("OUT_DIR").unwrap();
    let swizzle_file_path = Path::new(&out_dir).join("swizzle_operator_macro.rs");

    let data = format!(
"/// Generate glm/glsl style swizzle operators
macro_rules! impl_swizzle_functions {{
    ($vector_type2:ident, $S:ident, xy) => {{
{xy}
    }};
    ($vector_type2:ident, $vector_type3:ident, $S:ident, xyz) => {{
        impl_swizzle_functions!($vector_type2, $S, xy);
{xyz}
    }};
    ($vector_type2:ident, $vector_type3:ident, $vector_type4:ident, $S:ident, xyzw) => {{
        impl_swizzle_functions!($vector_type2, $vector_type3, $S, xyz);
{xyzw}
    }};
}}", xy=gen_swizzle_functions("xy"),
     xyz=gen_swizzle_functions("xyz"),
     xyzw=gen_swizzle_functions("xyzw"));
    let mut f = File::create(swizzle_file_path)
        .expect("Unable to create file that defines the swizzle operator macro.");
    f.write_all(data.as_bytes()).expect("Unable to write swizzle operator macro.");
}
