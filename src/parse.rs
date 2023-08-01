pub(crate) type Input<'a> = &'a [u8];
pub(crate) type ParseResult<'a, O> =
    nom::IResult<Input<'a>, O, nom::error::VerboseError<Input<'a>>>;
