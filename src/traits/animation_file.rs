use crate::traits::{AnimationKey, AnimationUser, FileId};

pub trait AnimationFile: 'static + Send + Sync {
    type FileId: FileId;
    type PackKey: AnimationKey;
    type AnimationKey: AnimationKey;
    type UserData: AnimationUser;

    fn to_file_name(file_id: &Self::FileId) -> &'static str;
    fn sprite_sheet_num(file_id: &Self::FileId) -> usize;
}
