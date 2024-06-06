mod Process;
mod arm7;
mod fc;
mod helpers;

#[cfg(test)]
mod tests {
    use super::arm7::*;
    const MOV: MOV = MOV {};

    #[test]
    fn test_mov1() {
        let en = MOV.get_encoding("mov r14, #0xFF");
        match en {
            Ok((encoding, ops)) => {
                assert_eq!(encoding, Encoding::ImmT1);
                println!("{}", ops.immed);
            }
            Err(mes) => panic!("{:?}", mes),
        }
    }
    #[test]
    fn test_mov2() {
        let en = MOV.get_encoding("mov r14, r12");
        match en {
            Ok((encoding, ops)) => {
                assert_eq!(encoding, Encoding::RegT1);
                println!("{}", MOV.encode(encoding, &ops));
            }
            Err(mes) => panic!("{:?}", mes),
        }
    }
    #[test]
    fn test_mov3() {
        let en = MOV.get_encoding("mov ");
        match en {
            Ok((encoding, ops)) => {
                panic!("Return Error: Not enough instructions.")
            }
            Err(mes) => println!("{:?}", mes),
        }
    }
    #[test]
    fn process_find_mnemonic_1() {
        let program = Program::new();
        assert_eq!(
            program.find_mnemonic(&"movs r9, r0 // comment".to_string()),
            Some("mov".into())
        );
        assert_eq!(
            program.find_mnemonic(&"movvs r9, r0 // comment".to_string()),
            Some("mov".into())
        );
        assert_eq!(
            program.find_mnemonic(&"mov r999 , r10930 // //sdcomment".to_string()),
            Some("mov".into())
        );
        assert_eq!(
            program.find_mnemonic(&"bad r999 , r10930 // //sdcomment".to_string()),
            None
        );
        assert_eq!(
            program.find_mnemonic(&"movgl r999 , r10930 // //sdcomment".to_string()),
            None
        );
        assert_eq!(
            program.find_mnemonic(&"movsvs r999 , r10930 // //sdcomment".to_string()),
            None
        );
    }
}
