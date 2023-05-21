#![no_main]

use libfuzzer_sys::arbitrary;
use libfuzzer_sys::arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;

use json_typegen_shared::{codegen, Options, OutputMode};

fuzz_target!(|data: &[u8]| {
    let mut u = Unstructured::new(data);
    let options = arbitrary_options(&mut u).unwrap();

    let name = u.arbitrary().unwrap();
    let source = Arbitrary::arbitrary_take_rest(u).unwrap();

    let _ = codegen(name, source, options);
});

fn arbitrary_options(u: &mut Unstructured) -> arbitrary::Result<Options> {
    let mut options = Options::default();

    options.output_mode = u
        .choose(&[
            OutputMode::Rust,
            OutputMode::Typescript,
            OutputMode::TypescriptTypeAlias,
            OutputMode::KotlinJackson,
            OutputMode::KotlinKotlinx,
            OutputMode::PythonPydantic,
            OutputMode::JsonSchema,
            OutputMode::Shape,
        ])?
        .to_owned();

    options.use_default_for_missing_fields = u.ratio(1, 4)?;

    Ok(options)
}
