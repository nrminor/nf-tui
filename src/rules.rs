use color_eyre::eyre::Result;

pub enum SupportedRules {
    IsFile,
    IsDir,
    IsNumber,
    IsString,
    NotNull,
}

#[allow(dead_code)]
impl SupportedRules {
    fn check_rule(_input: &str, supplied_rule: Option<SupportedRules>) -> Result<()> {
        match supplied_rule {
            Some(rule) => match rule {
                Self::IsFile => todo!(),
                Self::IsDir => todo!(),
                Self::IsNumber => todo!(),
                Self::IsString => todo!(),
                Self::NotNull => todo!(),
            },
            None => Ok(()),
        }
    }
}
