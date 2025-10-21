pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

pub struct SU8 {
    f1: u8,
    f2: u8,
    f3: u8,
}

//内存布局规则：
//1. 整个结构体必须按照最大field倍数，不够最后补padding。为了方便数组访问
// pub struct Padding1 {
//     f1: u32,        //4，是2自身倍数
//     f2: u16,        //2，4+2是1的倍数
//     f3: u8,         //1+6==7
//                     //整个struct 4+2+1不是最大field4的倍数，因此最后补1字节padding
//                     //size_of::<Padding1> = 4 + 2 + 1 + 1padding = 8
// }
pub struct Padding1 {
    f1: u32,
    f2: u16,
    f3: u8,
}

// #[repr(C)]           严格按照C风格分配内存，意味着不会调整field顺序
// pub struct Padding2 {
//     f1: u8,          //1，补充padding3, 1+3=4的倍数
//     f2: u32,         //4+4=8，是2的倍数
//     f3: u16,         //8+2=10，不是4的倍数
//                     //整个struct 1 + 3 + 4 + 2不是最大field4的倍数，因此最后补2字节padding
//                     //size_of::<Padding1> == 1 + 3padding + 4 + 2 + 2padding = 12
// }

#[repr(C)]
pub struct Padding2 {
    f1: u8,
    f3: u32,
    f2: u16,
}
// 默认按照Rust风格分配内存，会调整field顺序
// pub struct Padding3 {
//     f1: u8,
//     f2: u32,
//     f3: u16,
// }
// 会被优化为以下最小内存=8 bytes
// pub struct Padding3 {
//     f1: u32,
//     f2: u16,
//     f3: u8,
// }
pub struct Padding3 {
    f1: u8,
    f3: u32,
    f2: u16,
}


// #[repr(C)]           严格按照C风格分配内存，意味着不会调整field顺序
#[repr(C)]
pub struct Padding4 {
    f1: u8,         //1， up to ->2
    f2: u16,        //2+2=4, no up
    f3: u32,        //4+4=8, no up
    f4: [u8; 3],    //8+3=11, up to ->12
    f5: u16,        //12+2=14, no up
    f6: [u8; 3],    //14+3=17，up to->20
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use super::SU8;
    use super::*;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Type layout" section of The Rust Reference
        // https://doc.rust-lang.org/reference/type-layout.html for more information.
        assert_eq!(size_of::<Ticket>(), 72);
    }

    #[test]
    fn u8_size() {
        assert_eq!(size_of::<SU8>(), 3);
    }

    #[test]
    fn padding_size() {
        assert_eq!(size_of::<Padding1>(), 8);
        assert_eq!(size_of::<Padding2>(), 12);
        assert_eq!(size_of::<Padding3>(), 8);
        assert_eq!(size_of::<Padding4>(), 20);
    }
}
