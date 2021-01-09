// All of these try_alloc_with tests will fail with "fatal runtime error: stack overflow" unless
// LLVM manages to optimize the stack writes away.
//
// We only run them when debug_assertions are not set, as we expect them to fail outside release
// mode.

use bumpalo::Bump;

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn try_alloc_with_large_array() -> Result<(), ()> {
    let b = Bump::new();

    b.try_alloc_with(|| Ok([4u8; 10_000_000]))?;

    Ok(())
}

#[allow(dead_code)]
struct LargeStruct {
    small: usize,
    big1: [u8; 20_000_000],
    big2: [u8; 20_000_000],
    big3: [u8; 20_000_000],
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn try_alloc_with_large_struct() -> Result<(), ()> {
    let b = Bump::new();

    b.try_alloc_with(|| {
        Ok(LargeStruct {
            small: 1,
            big1: [2; 20_000_000],
            big2: [3; 20_000_000],
            big3: [4; 20_000_000],
        })
    })?;

    Ok(())
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn try_alloc_with_large_tuple() -> Result<(), ()> {
    let b = Bump::new();

    b.try_alloc_with(|| {
        Ok((
            1u32,
            LargeStruct {
                small: 2,
                big1: [3; 20_000_000],
                big2: [4; 20_000_000],
                big3: [5; 20_000_000],
            },
        ))
    })?;

    Ok(())
}

#[allow(clippy::large_enum_variant)]
enum LargeEnum {
    Small,
    #[allow(dead_code)]
    Large([u8; 10_000_000]),
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn try_alloc_with_large_enum() -> Result<(), ()> {
    let b = Bump::new();

    b.try_alloc_with(|| Ok(LargeEnum::Small))?;

    Ok(())
}
