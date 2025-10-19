use crate::status::Status;
use std::error::Error;

// We've seen how to declare modules in one of the earliest exercises, but
// we haven't seen how to extract them into separate files.
// Let's fix that now!
//
// In the simplest case, when the extracted module is a single file, it is enough to
// create a new file with the same name as the module and move the module content there.
// The module file should be placed in the same directory as the file that declares the module.
// In this case, `src/lib.rs`, thus `status.rs` should be placed in the `src` directory.
mod status;

// TODO: Add a new error variant to `TicketNewError` for when the status string is invalid.
//   When calling `source` on an error of that variant, it should return a `ParseStatusError` rather than `None`.

#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
    #[error("{0}")]
    InvalidStatus(#[from] status::ParseStatusError),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        // TODO: Parse the status string into a `Status` enum.
        let st = Status::try_from(status)?;

        Ok(Ticket {
            title,
            description,
            status: st,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};
    use thiserror::Error;

    #[test]
    fn invalid_status() {
        let err = Ticket::new(valid_title(), valid_description(), "invalid".into()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        );
        assert!(err.source().is_some());
    }

    //实现error嵌套，多个子error, 返回到一个父error中
    #[derive(Debug, Error)]
    enum CommonError {
        #[error("{0}")]
        #[from]
        InvalidInfo(SubError)
    }

    #[derive(Debug, Error)]
    enum SubError {
        #[error("sub error 1")]
        SubError1,
        #[error("sub error 2")]
        SubError2,
    }

    fn f_err1() -> Result<(), SubError> {
        Err(SubError::SubError1)
    }

    fn f_err2() -> Result<(), SubError> {
        Err(SubError::SubError2)
    }

    //?操作符做了很多事：
    //  *解包，判断是否有错误。没有则继续执行
    //  *有错误：
    //      * 会调用From Trait,把子错误转化为父错误
    //      * 会用Err把返回值封装为Result类型
    fn f_cmm(i: u8) -> Result<(), CommonError> {
        match i {
            1 => {
                f_err1()?;
                //必须加这个，因为？操作符只管理错误路径，正确路径需要返回值
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

