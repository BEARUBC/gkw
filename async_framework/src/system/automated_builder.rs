use std::{fs::{File, OpenOptions}, path::PathBuf};

use crate::prelude::SystemBuilder;

type AutomatedBuilderError = ();
type AutomatedBuilderResult<T> = Result<T, AutomatedBuilderError>;

#[allow(unused)]
pub(crate) fn build<M, R, A>(mut absolute_root: PathBuf, src: PathBuf) -> AutomatedBuilderResult<SystemBuilder<M, R, A>>
where
    M: 'static + Send,
    R: 'static,
    A: 'static,
{
    create_config_file(&mut absolute_root);

    todo!()
}

fn create_config_file(absolute_root: &mut PathBuf) -> File {
    absolute_root.push(".rp-config.rs");

    OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(absolute_root)
        .unwrap()
}
