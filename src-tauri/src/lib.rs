mod fc;
mod arm7;

use arm7::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mov1() {
        let en = MOV::get_encoding("mov r14, #0xFF");
        match en {
            Ok((encoding, ops)) => {
                assert_eq!(encoding, Encoding::ImmT1);
                println!("{}", ops.immed);
            },
            Err(mes) => panic!("{}", mes),
        }
    }
    #[test]
    fn test_mov2() {
        let en = MOV::get_encoding("mov r14, r12");
        match en {
            Ok((encoding, ops)) => {
                assert_eq!(encoding, Encoding::RegT1);
                println!("{}", MOV::encode(encoding, &ops));
            },
            Err(mes) => panic!("{}", mes),
        }
    }
    #[test]
    fn test_mov3() {
        let en = MOV::get_encoding("mov ");
        match en {
            Ok((encoding, ops)) => {
                panic!("Return Error: Not enough instructions.")
            },
            Err(mes) => println!("{}", mes),
        }
    }

}