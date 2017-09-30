use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
use std::string::String;

/// Generate the name of the swizzle function and what it returns.
/// NOTE: This function assumes that variables are in ASCII format
fn gen_swizzle_nth<'a>(variables: &'a str, mut i: usize) -> Option<(String, String)> {
    debug_assert!(i > 0); // zeroth permutation is empty
    let mut swizzle_impl = String::new();
    let mut swizzle = String::new();
    let n = variables.len()+1;
    for _ in 0..variables.len() {
        if i == 0 { break; }
        if i % n == 0 { return None; }
        let c = variables.as_bytes()[i%n - 1] as char;
        swizzle.push(c);
        swizzle_impl.push_str(&format!("self.{}, ", c));
        i = i/n;
    }
    Some((swizzle, swizzle_impl))
}

/// NOTE: This function assumes that variables are in ASCII format
fn gen_swizzle_functions(variables: &'static str) -> String {
    let n = variables.len();
    let mut result = String::new();
    let nn = (n+1).pow(n as u32);
    for i in 1..nn {
        if let Some((swizzle_name, swizzle_impl)) = gen_swizzle_nth(variables, i) {
            let vec_type = format!("$vector_type{}", swizzle_name.len());
            result.push_str(
                &format!("        #[inline] pub fn {0}(&self) -> {2}<$S> {{ {2}::new({1}) }}\n",
                swizzle_name, swizzle_impl, vec_type));
        }
    }
    result
}

/// This script generates the macro for building swizzle operators for multidimensional
/// vectors and points. This macro is included in macros.rs
fn main() {
    // save the file to output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let swizzle_file_path = Path::new(&out_dir).join("swizzle_operator_macro.rs");

    // This is the string representing the generated macro
    let data = format!(
"/// Generate glm/glsl style swizzle operators
macro_rules! impl_swizzle_functions {{
    ($vector_type1:ident, $S:ident, x) => {{
{x}
    }};
    ($vector_type1:ident, $vector_type2:ident, $S:ident, xy) => {{
{xy}
    }};
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $S:ident, xyz) => {{
{xyz}
    }};
    ($vector_type1:ident, $vector_type2:ident, $vector_type3:ident, $vector_type4:ident, $S:ident, xyzw) => {{
{xyzw}
    }};
}}", x=gen_swizzle_functions("x"),
     xy=gen_swizzle_functions("xy"),
     xyz=gen_swizzle_functions("xyz"),
     xyzw=gen_swizzle_functions("xyzw"));
    let mut f = File::create(swizzle_file_path)
        .expect("Unable to create file that defines the swizzle operator macro.");
    f.write_all(data.as_bytes()).expect("Unable to write swizzle operator macro.");
}
