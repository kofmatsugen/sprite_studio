pub trait AnimationFile {
    fn to_file_name(&self) -> &'static str;
    fn sprite_sheet_num(&self) -> usize;
}
