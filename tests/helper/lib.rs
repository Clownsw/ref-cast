use ref_cast::RefCastCustom;

#[derive(RefCastCustom)]
#[repr(transparent)]
pub struct Struct(str);
