pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn u16_ref_size() {
        assert_eq!(size_of::<&u16>(), 8);
    }

    #[test]
    fn u64_mut_ref_size() {
        assert_eq!(size_of::<&mut u64>(), 8);
    }

    #[test]
    fn ticket_ref_size() {
        assert_eq!(size_of::<&Ticket>(), 8);
    }

    //fat pointer，胖指针和普通指针不一样，即存储位置，还存储长度
    //因为编译期无法得知具体的长度，，胖指针比如&str, 切片等
    #[test]
    fn str_size() {
        assert_eq!(size_of::<&String>(), 8);        //普通指针
        assert_eq!(size_of::<&str>(), 16);          //胖指针
    }
}
