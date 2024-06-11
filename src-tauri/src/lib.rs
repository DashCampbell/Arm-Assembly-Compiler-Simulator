mod Process;
mod arm7;
mod error;
mod fc;
mod helpers;
mod instructions;

#[cfg(test)]
mod tests {
    use super::arm7::*;
    use crate::helpers as hp;

    const MOV: MOV = MOV {};

    #[test]
    fn test_mov1() {
        let en = MOV.get_operands(&MnemonicExtension::new(), "mov r14, #0x20");
        match en {
            Ok(Operands::Rd_immed { Rd, immed }) => {
                assert_eq!(Rd, 14);
                assert_eq!(immed, 32);
            }
            _ => panic!(),
        }
    }
    #[test]
    fn test_mov3() {
        let en = MOV.get_operands(&MnemonicExtension::new(), "mov ");
        match en {
            Err(mes) => println!("{:?}", mes),
            _ => panic!("Return Error: Not enough instructions."),
        }
    }

    #[test]
    fn get_all_numbers() {
        assert_eq!(hp::get_all_numbers("movseq r0, #10"), Ok(vec![0, 10]));
        assert_eq!(hp::get_all_numbers("adds r4, #-1"), Ok(vec![4, u32::MAX]));
        assert_eq!(
            hp::get_all_numbers("ldr sp, #-0b100"),
            Ok(vec![13, u32::MAX - 3])
        );
        assert_eq!(
            hp::get_all_numbers("ldr r1, lr, #-0xa"),
            Ok(vec![1, 14, u32::MAX - 9])
        );
        assert_eq!(
            hp::get_all_numbers("ldr r10, [pc, #0x20]"),
            Ok(vec![10, 15, 32])
        );
        assert_eq!(
            hp::get_all_numbers("ldr r10, r0, r14, pc, #255"),
            Ok(vec![10, 0, 14, 15, 255])
        );
        assert_eq!(
            hp::get_all_numbers("mov #afff"),
            Err(vec!["#afff is not a valid immediate value.".to_string()])
        );
    }
    #[test]
    fn is_rd_immed() {
        // test is_Rd_immed function
        assert_eq!(hp::is_Rd_immed("mov  r0, #4"), true);
        assert_eq!(hp::is_Rd_immed("mov  r0,"), false);
        assert_eq!(hp::is_Rd_immed("mov  #4"), false);
        assert_eq!(hp::is_Rd_immed("movs  r2, #0b1100"), true);
        assert_eq!(hp::is_Rd_immed("moveq  r3, #0xffff"), true);
        assert_eq!(hp::is_Rd_immed("mov.w  r16,  #-0xa"), true);
        assert_eq!(hp::is_Rd_immed("movsvs.w  r12,#-4"), true);
        assert_eq!(hp::is_Rd_immed("mo  r12, #-4"), true);
    }
    #[test]
    fn is_rd_rm() {
        assert_eq!(hp::is_Rd_Rm("mov r0, r1"), true);
        assert_eq!(hp::is_Rd_Rm("mov sp, pc"), true);
        assert_eq!(hp::is_Rd_Rm("movscc pc, sp"), true);
        assert_eq!(hp::is_Rd_Rm("movs r0, r21"), true);
        assert_eq!(hp::is_Rd_Rm("moveq   r30,r1"), true);
        assert_eq!(hp::is_Rd_Rm("mov r0, r"), false);
        assert_eq!(hp::is_Rd_Rm("movsvs r0, r1"), true);
        assert_eq!(hp::is_Rd_Rm("movsvs r0, #4"), false);
    }
    #[test]
    fn is_rd_rn_immed() {
        assert_eq!(hp::is_Rd_Rn_immed("add r0, r1, #12"), true);
        assert_eq!(hp::is_Rd_Rn_immed("add.w r0,r1,#0xa"), true);
        assert_eq!(hp::is_Rd_Rn_immed("adds r12, r18,#0xa"), true);
        assert_eq!(hp::is_Rd_Rn_immed("addcc    r12 , r18 , #0b11"), true);
        assert_eq!(hp::is_Rd_Rn_immed("adds r12, r,#0xa"), false);
        assert_eq!(hp::is_Rd_Rn_immed("adds r12, r"), false);
    }
    #[test]
    fn is_rd_rn_rm() {
        assert_eq!(hp::is_Rd_Rn_Rm("add r0, r1, sp"), true);
        assert_eq!(hp::is_Rd_Rn_Rm("add.w r0,r1, pc"), true);
        assert_eq!(hp::is_Rd_Rn_Rm("adds r12, r18, r14"), true);
        assert_eq!(hp::is_Rd_Rn_Rm("addcc    r12 , r18 , r0"), true);
        assert_eq!(hp::is_Rd_Rn_Rm("adds r12, r,#0xa"), false);
        assert_eq!(hp::is_Rd_Rn_Rm("adds r12, r"), false);
    }

    #[test]
    fn process_find_mnemonic_1() {
        let program = Program::new();
        assert_eq!(
            program.find_mnemonic(&"movs r9, r0 // comment".to_string()),
            Some((
                "mov".into(),
                MnemonicExtension {
                    cc: None,
                    s: true,
                    w: false
                }
            ))
        );
        assert_eq!(
            program.find_mnemonic(&"movvs r9, r0 // comment".to_string()),
            Some((
                "mov".into(),
                MnemonicExtension {
                    cc: Some(ConditionCode::VS),
                    s: false,
                    w: false
                }
            ))
        );
        assert_eq!(
            program.find_mnemonic(&"mov r999 , r10930 // //sdcomment".to_string()),
            Some((
                "mov".into(),
                MnemonicExtension {
                    cc: None,
                    s: false,
                    w: false
                }
            ))
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
            Some((
                "mov".into(),
                MnemonicExtension {
                    cc: Some(ConditionCode::VS),
                    s: true,
                    w: false,
                }
            ))
        );
        assert_eq!(
            program.find_mnemonic(&"movsvs.w r999 , r10930 // //sdcomment".to_string()),
            Some((
                "mov".into(),
                MnemonicExtension {
                    cc: Some(ConditionCode::VS),
                    s: true,
                    w: true,
                }
            ))
        );
        assert_eq!(
            program.find_mnemonic(&"mov.w r999 , #-40 // //sdcomment".to_string()),
            Some((
                "mov".into(),
                MnemonicExtension {
                    cc: None,
                    s: false,
                    w: true,
                }
            ))
        );
        assert_eq!(
            program.find_mnemonic(&"adds r999 , #-40 // //sdcomment".to_string()),
            Some((
                "add".into(),
                MnemonicExtension {
                    cc: None,
                    s: true,
                    w: false,
                }
            ))
        );
    }
}
