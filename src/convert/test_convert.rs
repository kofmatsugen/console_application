use amethyst_sprite_studio::traits::animation_file::AnimationFile;
use arraystring::{typenum::U255, ArrayString};

pub struct TestFile;

impl AnimationFile for TestFile {
    type FileId = ArrayString<U255>;
    type PackKey = ArrayString<U255>;
    type AnimationKey = ArrayString<U255>;
    type UserData = ();

    fn to_file_name(_: &Self::FileId) -> &'static str {
        ""
    }
    fn sprite_sheet_num(_: &Self::FileId) -> usize {
        0
    }
}
