use std::collections::HashMap;
/*macro_rules! opcodes {
    ($($name: ident,$bytes: expr,$sname: expr),*) => {

        $(
            #[allow(non_upper_case_globals)]
            pub const $name: &'static [u8] = &$bytes;
        )*

        lazy_static::lazy_static! {
            pub static ref OPCODE_TABLE: HashMap<Vec<u8>,String> = {
                let mut map = HashMap::new();
                $(
                    map.insert($bytes.iter().map(|b| *b).collect::<Vec<u8>>(), $sname.to_string());
                )*
                map
            };

            pub static ref OPCODE_TO_BYTES: HashMap<String,Vec<u8>> = {
                let mut map = HashMap::new();
                $(
                    map.insert($sname.to_string(), $bytes.iter().map(|b| *b).collect::<Vec<u8>>());
                )*

                map
            };
        }
    };
}

opcodes! {
    add,[0x58],"add",
    add_ovf,[0xd6],"add.ovf",
    add_ovf_un,[0xd7],"add.ovf.un",
    and,[0x5f],"and",
    arglist,[0xfe,0x00],"arglist",
    beq,[0x3b],"beq",
    beq_s,[0x2e],"beq.s",
    bge,[0x3c],"bge",
    bge_s,[0xf2],"bge.s",
    bge_un,[0x41],"bge.un",
    bge_un_s,[0x34],"bge.un.s",
    bgt,[0x3d],"bgt",
    bgt_s,[0x30],"bgt.s",
    bgt_un,[0x42],"bgt.un",
    bgt_un_s,[0x35],"bgt.un.s",
    ble,[0x3e],"ble",
    ble_s,[0x31],"ble.s",
    ble_un,[0x43],"ble.un",
    ble_un_s,[0x36],"ble.un.s",
    blt,[0x3f],"blt",
    blt_s,[0x32],"blt.s",
    blt_un,[0x44],"blt.un",
    blt_un_s,[0x37],"blt.un.s",
    bne_un,[0x40],"bne.un",
    bne_un_s,[0x33],"bne.un.s",
    box_,[0x8c],"box",
    br,[0x38],"br",
    br_s,[0x2b],"bs.s",
    break_,[0x01],"break",
    brfalse,[0x39],"brfalse",
    brfalse_s,[0x2c],"brfalse.s",
    brinst,[0x3a],"brinst",
    brinst_s,[0x2d],"brinst.s",
    brnull,[0x39],"brnull",
    brnull_s,[0x2c],"brnull.s",
    brtrue,[0x3a],"brtrue",
    brtrue_s,[0x2d],"brtrue.s",
    brzero,[0x39],"brzero",
    brzero_s,[0x2c],"brzero.s",
    call,[0x28],"call",
    calli,[0x29],"calli",
    callvirt,[0x6f],"callvirt",
    castclass,[0x74],"castclass",
    ceq,[0xfe,0x01],"ceq",
    cgt,[0xfe,0x02],"cgt",
    cgt_un,[0xfe,0x03],"cgt.un",
    ckfinite,[0xc3],"ckfinite",
    clt,[0xfe,0x04],"clt",
    clt_un,[0xfe,0x05],"clt.un",
    constrained,[0xfe,0x16],"constrained",
    conv_i,[0xd3],"conv.i",
    conv_i1,[0x67],"conv.i1",
    conv_i2,[0x68],"conv.i2",
    conv_i4,[0x69],"conv.i4",
    conv_i8,[0x6a],"conv.i8",
    conv_ovf_i,[0xd4],"conv.ovf.i",
    conv_ovf_i_un,[0x8a],"conv.ovf.i.un",
    conv_ovf_i1,[0xb3],"conv.ovf.i1",
    conv_ovf_i1_un,[0x82],"conv.ovf.i1.un",
    conv_ovf_i2,[0xb5],"conv.ovf.i2",
    conv_ovf_i2_un,[0x83],"conv.ovf.i2.un",
    conv_ovf_i4,[0xb7],"conv.ovf.i4",
    conv_ovf_i4_un,[0x84],"conv.ovf.i4.un",
    conv_ovf_i8,[0xb9],"conv.ovf.i8",
    conv_ovf_i8_un,[0x85],"convf.ovf.i8.un",
    conv_ovf_u,[0xd5],"convf.ovf.u",
    conv_ovf_u_un,[0x8b],"conv.ovf.u.un",
    conv_ovf_u1,[0xb4],"conv.ovf.u1",
    conv_ovf_u1_un,[0x86],"conv.ovf.u1.un",
    conv_ovf_u2,[0xb6],"conv.ovf.u2",
    conv_ovf_u2_un,[0x87],"conv.ovf.u2.un",
    conv_ovf_u4,[0xb8],"conv.ovf.u4",
    conv_ovf_u4_un,[0x88],"convf.ovf.u4.un",
    conv_ovf_u8,[0xba],"conv.ovf.u8",
    conv_ovf_u8_un,[0x89],"conv.ovf.u8.un",
    conv_r_un,[0x76],"conv.r.un",
    conv_r4,[0x6b],"conv.r4",
    conv_r8,[0x6c],"conv.r8",
    conv_u,[0xe0],"conv.u",
    conv_u1,[0xd2],"conv.u1",
    conv_u2,[0xd1],"conv.u2",
    conv_u4,[0x6d],"conv.u4",
    conv_u8,[0x6e],"conv.u8",
    cpblk,[0xfe,0x17],"cpblk",
    cpobj,[0x70],"cpobj",
    div,[0x5b],"div",
    div_un,[0x5c],"div.un",
    dup,[0x25],"dup",
    endfault,[0xdc],"endfault",
    endfilter,[0xfe,0x11],"endfilter",
    endfinally,[0xdc],"endfinally",
    initblk,[0xfe,0x18],"initblk",
    initobj,[0xfe,0x15],"initobj",
    isint,[0x75],"isinst",
    jmp,[0x27],"jmp",
    ldarg,[0xfe,0x09],"ldarg",
    ldarg_0,[0x02],"ldarg.0",
    ldarg_1,[0x03],"ldarg.1",
    ldarg_2,[0x04],"ldarg.2",
    ldarg_3,[0x05],"ldarg.3",
    ldarg_s,[0x0e],"ldarg.s",
    ldarga,[0xfe,0x0a],"ldarga",
    ldarga_s,[0x0f],"ldarga.s",
    ldc_i4,[0x20],"ldc.i4",
    ldc_i4_0,[0x16],"ldc.i4.0",
    ldc_i4_1,[0x17],"ldc.i4.1",
    ldc_i4_2,[0x18],"ldc.i4.2",
    ldc_i4_3,[0x19],"ldc.i4.3",
    ldc_i4_4,[0x1a],"ldc.i4.4",
    ldc_i4_5,[0x1b],"ldc.i4.5",
    ldc_i4_6,[0x1c],"ldc.i4.6",
    ldc_i4_7,[0x1d],"ldc.i4.7",
    ldc_i4_8,[0x1e],"ldc.i4.8",
    ldc_i4_m1,[0x15],"ldc.i4.m1",
    ldc_i4_s,[0x1f],"ldc.i4.s",
    ldc_i8,[0x21],"ldc.i8",
    ldc_r4,[0x22],"ldc.r4",
    ldc_r8,[0x23],"ldc.r8",
    ldelem,[0xa3],"ldelem",
    ldelem_i,[0x97],"ldelem.i",
    ldelem_i1,[0x90],"ldelem.i1",
    ldelem_i2,[0x92],"ldelem.i2",
    ldelem_i4,[0x94],"ldelem.i4",
    ldelem_i8,[0x96],"ldelem.i8",
    ldelem_r4,[0x98],"ldelem.r4",
    ldelem_r8,[0x99],"ldelem.r8",
    ldelem_ref,[0x9a],"ldelem.ref",
    ldelem_u1,[0x91],"ldelem.u1",
    ldelem_u2,[0x93],"ldelem.u2",
    ldelem_u4,[0x95],"ldelem.u4",
    ldelem_u8,[0x96],"ldelem.u8",
    ldelema,[0x8f],"ldelema",
    ldfld,[0x7b],"ldfld",
    ldflda,[0x7c],"ldflda",
    ldftn,[0xfe,0x06],"ldftn",
    ldind_i,[0x4d],"ldind.i",
    ldind_i1,[0x46],"ldind.i1",
    ldind_i2,[0x48],"ldind.i2",
    ldind_i4,[0x4a],"ldind.i4",
    ldind_i8,[0x4c],"ldind.i8",
    ldind_r4,[0x4e],"ldind.r4",
    ldind_r8,[0x4f],"ldind.r8",
    ldind_ref,[0x50],"ldind.ref",
    ldind_u1,[0x47],"ldind.u1",
    ldind_u2,[0x49],"ldind.u2",
    ldind_u4,[0x4b],"ldind.u4",
    ldind_u8,[0x4c],"ldind.i8",
    ldlen,[0x8e],"ldlen",
    ldloc_0,[0x06],"ldloc.0",
    ldloc_1,[0x07],"ldloc.1",
    ldloc_2,[0x08],"ldloc.2",
    ldloc_3,[0x09],"ldloc.3",
    ldloc_s,[0x11],"ldloc.s",
    ldloca,[0xfe,0x0d],"ldloca",
    ldloca_s,[0x12],"ldloca.s",
    ldnull,[0x14],"ldnull",
    ldobj,[0x71],"ldobj",
    ldsfld,[0x7e],"ldsfld",
    ldsflda,[0x7f],"ldsflda",
    ldstr,[0x72],"ldstr",
    ldtoken,[0xd0],"ldtoken",
    ldvirtfn,[0xfe,0x07],"ldvirtfn",
    leave,[0xdd],"leave",
    leave_s,[0xde],"leave.s",
    localloc,[0xfe,0x0f],"localloc",
    mkrefany,[0xc6],"mkrefany",
    mul,[0x5a],"mul",
    mul_ovf,[0xd8],"mul.ovf",
    mul_ovf_un,[0xd9],"mul.ovf.un",
    neg,[0x65],"neg",
    newarr,[0x8d],"newarr",
    newobj,[0x73],"newobj",
    no_,[0xfe,0x19],"no. {typecheck,rangecheck,nullcheck}",
    nop,[0x00],"nop",
    not,[0x66],"not",
    or,[0x60],"or",
    pop,[0x26],"pop",
    readonly,[0xfe,0x1e],"readonly",
    refanytype,[0xfe,0x1d],"refanytype",
    refanyval,[0xc2],"refanyval",
    rem,[0x5d],"rem",
    rem_un,[0x5e],"rem.un",
    ret,[0x2a],"ret",
    rethrow,[0xfe,0x1a],"rethrow",
    shl,[0x62],"shl",
    shr,[0x63],"shr",
    shr_un,[0x64],"shr.un",
    sizeof,[0xfe,0x1c],"sizeof",
    starg,[0xfe,0x0b],"starg",
    starg_s,[0x10],"starg.s",
    stelem,[0xa4],"stelem",
    stelem_i,[0x9b],"stelem.i",
    stelem_i1,[0x9c],"stelem.i1",
    stelem_i2,[0x9d],"stelem.i2",
    stelem_i4,[0x9e],"stelem.i4",
    stelem_i8,[0x9f],"stelem.i8",
    stelem_r4,[0xa0],"stelem.r4",
    stelem_r8,[0xa1],"stelem.r8",
    stelem_ref,[0xa2],"stelem.ref",
    stfld,[0x7d],"stfld",
    stind_i,[0xdf],"stind.i",
    stind_i1,[0x52],"stind.i1",
    stind_i2,[0x53],"stind.i2",
    stind_i4,[0x54],"stind.i4",
    stind_i8,[0x55],"stind.i8",
    stind_r4,[0x56],"stind.r4",
    stind_r8,[0x57],"stind.r8",
    stind_ref,[0x51],"stind.ref",
    stloc,[0xfe,0x0e],"stloc",
    stloc_0,[0x0a],"stloc.0",
    stloc_1,[0x0b],"stloc.1",
    stloc_2,[0x0c],"stloc.2",
    stloc_3,[0x0d],"stloc.3",
    stloc_s,[0x13],"stloc.s",
    stobj,[0x81],"stobj",
    stsfld,[0x80],"stsfld",
    sub,[0x59],"sub",
    sub_ovf,[0xda],"sub.ovf",
    sub_ovf_un,[0xdb],"sub.ovf.un",
    switch,[0x45],"switch",
    tailcall,[0xfe,0x14],"tail",
    throw,[0x7a],"throw",
    unaligned,[0xfe,0x12],"unaligned",
    unbox,[0x79],"unbox",
    unbox_any,[0xa5],"unbox.any",
    volatile,[0xfe,0x13],"volatile",
    xor,[0x61],"xor"

}
*/

#[allow(non_upper_case_globals)]
pub const add: &'static [u8] = &[88];
#[allow(non_upper_case_globals)]
pub const add_ovf: &'static [u8] = &[214];
#[allow(non_upper_case_globals)]
pub const add_ovf_un: &'static [u8] = &[215];
#[allow(non_upper_case_globals)]
pub const and: &'static [u8] = &[95];
#[allow(non_upper_case_globals)]
pub const arglist: &'static [u8] = &[254, 0];
#[allow(non_upper_case_globals)]
pub const beq: &'static [u8] = &[59];
#[allow(non_upper_case_globals)]
pub const beq_s: &'static [u8] = &[46];
#[allow(non_upper_case_globals)]
pub const bge: &'static [u8] = &[60];
#[allow(non_upper_case_globals)]
pub const bge_s: &'static [u8] = &[242];
#[allow(non_upper_case_globals)]
pub const bge_un: &'static [u8] = &[65];
#[allow(non_upper_case_globals)]
pub const bge_un_s: &'static [u8] = &[52];
#[allow(non_upper_case_globals)]
pub const bgt: &'static [u8] = &[61];
#[allow(non_upper_case_globals)]
pub const bgt_s: &'static [u8] = &[48];
#[allow(non_upper_case_globals)]
pub const bgt_un: &'static [u8] = &[66];
#[allow(non_upper_case_globals)]
pub const bgt_un_s: &'static [u8] = &[53];
#[allow(non_upper_case_globals)]
pub const ble: &'static [u8] = &[62];
#[allow(non_upper_case_globals)]
pub const ble_s: &'static [u8] = &[49];
#[allow(non_upper_case_globals)]
pub const ble_un: &'static [u8] = &[67];
#[allow(non_upper_case_globals)]
pub const ble_un_s: &'static [u8] = &[54];
#[allow(non_upper_case_globals)]
pub const blt: &'static [u8] = &[63];
#[allow(non_upper_case_globals)]
pub const blt_s: &'static [u8] = &[50];
#[allow(non_upper_case_globals)]
pub const blt_un: &'static [u8] = &[68];
#[allow(non_upper_case_globals)]
pub const blt_un_s: &'static [u8] = &[55];
#[allow(non_upper_case_globals)]
pub const bne_un: &'static [u8] = &[64];
#[allow(non_upper_case_globals)]
pub const bne_un_s: &'static [u8] = &[51];
#[allow(non_upper_case_globals)]
pub const box_: &'static [u8] = &[140];
#[allow(non_upper_case_globals)]
pub const br: &'static [u8] = &[56];
#[allow(non_upper_case_globals)]
pub const br_s: &'static [u8] = &[43];
#[allow(non_upper_case_globals)]
pub const break_: &'static [u8] = &[1];
#[allow(non_upper_case_globals)]
pub const brfalse: &'static [u8] = &[57];
#[allow(non_upper_case_globals)]
pub const brfalse_s: &'static [u8] = &[44];
#[allow(non_upper_case_globals)]
pub const brinst: &'static [u8] = &[58];
#[allow(non_upper_case_globals)]
pub const brinst_s: &'static [u8] = &[45];
#[allow(non_upper_case_globals)]
pub const brnull: &'static [u8] = &[57];
#[allow(non_upper_case_globals)]
pub const brnull_s: &'static [u8] = &[44];
#[allow(non_upper_case_globals)]
pub const brtrue: &'static [u8] = &[58];
#[allow(non_upper_case_globals)]
pub const brtrue_s: &'static [u8] = &[45];
#[allow(non_upper_case_globals)]
pub const brzero: &'static [u8] = &[57];
#[allow(non_upper_case_globals)]
pub const brzero_s: &'static [u8] = &[44];
#[allow(non_upper_case_globals)]
pub const call: &'static [u8] = &[40];
#[allow(non_upper_case_globals)]
pub const calli: &'static [u8] = &[41];
#[allow(non_upper_case_globals)]
pub const callvirt: &'static [u8] = &[111];
#[allow(non_upper_case_globals)]
pub const castclass: &'static [u8] = &[116];
#[allow(non_upper_case_globals)]
pub const ceq: &'static [u8] = &[254, 1];
#[allow(non_upper_case_globals)]
pub const cgt: &'static [u8] = &[254, 2];
#[allow(non_upper_case_globals)]
pub const cgt_un: &'static [u8] = &[254, 3];
#[allow(non_upper_case_globals)]
pub const ckfinite: &'static [u8] = &[195];
#[allow(non_upper_case_globals)]
pub const clt: &'static [u8] = &[254, 4];
#[allow(non_upper_case_globals)]
pub const clt_un: &'static [u8] = &[254, 5];
#[allow(non_upper_case_globals)]
pub const constrained: &'static [u8] = &[254, 22];
#[allow(non_upper_case_globals)]
pub const conv_i: &'static [u8] = &[211];
#[allow(non_upper_case_globals)]
pub const conv_i1: &'static [u8] = &[103];
#[allow(non_upper_case_globals)]
pub const conv_i2: &'static [u8] = &[104];
#[allow(non_upper_case_globals)]
pub const conv_i4: &'static [u8] = &[105];
#[allow(non_upper_case_globals)]
pub const conv_i8: &'static [u8] = &[106];
#[allow(non_upper_case_globals)]
pub const conv_ovf_i: &'static [u8] = &[212];
#[allow(non_upper_case_globals)]
pub const conv_ovf_i_un: &'static [u8] = &[138];
#[allow(non_upper_case_globals)]
pub const conv_ovf_i1: &'static [u8] = &[179];
#[allow(non_upper_case_globals)]
pub const conv_ovf_i1_un: &'static [u8] = &[130];
#[allow(non_upper_case_globals)]
pub const conv_ovf_i2: &'static [u8] = &[181];
#[allow(non_upper_case_globals)]
pub const conv_ovf_i2_un: &'static [u8] = &[131];
#[allow(non_upper_case_globals)]
pub const conv_ovf_i4: &'static [u8] = &[183];
#[allow(non_upper_case_globals)]
pub const conv_ovf_i4_un: &'static [u8] = &[132];
#[allow(non_upper_case_globals)]
pub const conv_ovf_i8: &'static [u8] = &[185];
#[allow(non_upper_case_globals)]
pub const conv_ovf_i8_un: &'static [u8] = &[133];
#[allow(non_upper_case_globals)]
pub const conv_ovf_u: &'static [u8] = &[213];
#[allow(non_upper_case_globals)]
pub const conv_ovf_u_un: &'static [u8] = &[139];
#[allow(non_upper_case_globals)]
pub const conv_ovf_u1: &'static [u8] = &[180];
#[allow(non_upper_case_globals)]
pub const conv_ovf_u1_un: &'static [u8] = &[134];
#[allow(non_upper_case_globals)]
pub const conv_ovf_u2: &'static [u8] = &[182];
#[allow(non_upper_case_globals)]
pub const conv_ovf_u2_un: &'static [u8] = &[135];
#[allow(non_upper_case_globals)]
pub const conv_ovf_u4: &'static [u8] = &[184];
#[allow(non_upper_case_globals)]
pub const conv_ovf_u4_un: &'static [u8] = &[136];
#[allow(non_upper_case_globals)]
pub const conv_ovf_u8: &'static [u8] = &[186];
#[allow(non_upper_case_globals)]
pub const conv_ovf_u8_un: &'static [u8] = &[137];
#[allow(non_upper_case_globals)]
pub const conv_r_un: &'static [u8] = &[118];
#[allow(non_upper_case_globals)]
pub const conv_r4: &'static [u8] = &[107];
#[allow(non_upper_case_globals)]
pub const conv_r8: &'static [u8] = &[108];
#[allow(non_upper_case_globals)]
pub const conv_u: &'static [u8] = &[224];
#[allow(non_upper_case_globals)]
pub const conv_u1: &'static [u8] = &[210];
#[allow(non_upper_case_globals)]
pub const conv_u2: &'static [u8] = &[209];
#[allow(non_upper_case_globals)]
pub const conv_u4: &'static [u8] = &[109];
#[allow(non_upper_case_globals)]
pub const conv_u8: &'static [u8] = &[110];
#[allow(non_upper_case_globals)]
pub const cpblk: &'static [u8] = &[254, 23];
#[allow(non_upper_case_globals)]
pub const cpobj: &'static [u8] = &[112];
#[allow(non_upper_case_globals)]
pub const div: &'static [u8] = &[91];
#[allow(non_upper_case_globals)]
pub const div_un: &'static [u8] = &[92];
#[allow(non_upper_case_globals)]
pub const dup: &'static [u8] = &[37];
#[allow(non_upper_case_globals)]
pub const endfault: &'static [u8] = &[220];
#[allow(non_upper_case_globals)]
pub const endfilter: &'static [u8] = &[254, 17];
#[allow(non_upper_case_globals)]
pub const endfinally: &'static [u8] = &[220];
#[allow(non_upper_case_globals)]
pub const initblk: &'static [u8] = &[254, 24];
#[allow(non_upper_case_globals)]
pub const initobj: &'static [u8] = &[254, 21];
#[allow(non_upper_case_globals)]
pub const isint: &'static [u8] = &[117];
#[allow(non_upper_case_globals)]
pub const jmp: &'static [u8] = &[39];
#[allow(non_upper_case_globals)]
pub const ldarg: &'static [u8] = &[254, 9];
#[allow(non_upper_case_globals)]
pub const ldarg_0: &'static [u8] = &[2];
#[allow(non_upper_case_globals)]
pub const ldarg_1: &'static [u8] = &[3];
#[allow(non_upper_case_globals)]
pub const ldarg_2: &'static [u8] = &[4];
#[allow(non_upper_case_globals)]
pub const ldarg_3: &'static [u8] = &[5];
#[allow(non_upper_case_globals)]
pub const ldarg_s: &'static [u8] = &[14];
#[allow(non_upper_case_globals)]
pub const ldarga: &'static [u8] = &[254, 10];
#[allow(non_upper_case_globals)]
pub const ldarga_s: &'static [u8] = &[15];
#[allow(non_upper_case_globals)]
pub const ldc_i4: &'static [u8] = &[32];
#[allow(non_upper_case_globals)]
pub const ldc_i4_0: &'static [u8] = &[22];
#[allow(non_upper_case_globals)]
pub const ldc_i4_1: &'static [u8] = &[23];
#[allow(non_upper_case_globals)]
pub const ldc_i4_2: &'static [u8] = &[24];
#[allow(non_upper_case_globals)]
pub const ldc_i4_3: &'static [u8] = &[25];
#[allow(non_upper_case_globals)]
pub const ldc_i4_4: &'static [u8] = &[26];
#[allow(non_upper_case_globals)]
pub const ldc_i4_5: &'static [u8] = &[27];
#[allow(non_upper_case_globals)]
pub const ldc_i4_6: &'static [u8] = &[28];
#[allow(non_upper_case_globals)]
pub const ldc_i4_7: &'static [u8] = &[29];
#[allow(non_upper_case_globals)]
pub const ldc_i4_8: &'static [u8] = &[30];
#[allow(non_upper_case_globals)]
pub const ldc_i4_m1: &'static [u8] = &[21];
#[allow(non_upper_case_globals)]
pub const ldc_i4_s: &'static [u8] = &[31];
#[allow(non_upper_case_globals)]
pub const ldc_i8: &'static [u8] = &[33];
#[allow(non_upper_case_globals)]
pub const ldc_r4: &'static [u8] = &[34];
#[allow(non_upper_case_globals)]
pub const ldc_r8: &'static [u8] = &[35];
#[allow(non_upper_case_globals)]
pub const ldelem: &'static [u8] = &[163];
#[allow(non_upper_case_globals)]
pub const ldelem_i: &'static [u8] = &[151];
#[allow(non_upper_case_globals)]
pub const ldelem_i1: &'static [u8] = &[144];
#[allow(non_upper_case_globals)]
pub const ldelem_i2: &'static [u8] = &[146];
#[allow(non_upper_case_globals)]
pub const ldelem_i4: &'static [u8] = &[148];
#[allow(non_upper_case_globals)]
pub const ldelem_i8: &'static [u8] = &[150];
#[allow(non_upper_case_globals)]
pub const ldelem_r4: &'static [u8] = &[152];
#[allow(non_upper_case_globals)]
pub const ldelem_r8: &'static [u8] = &[153];
#[allow(non_upper_case_globals)]
pub const ldelem_ref: &'static [u8] = &[154];
#[allow(non_upper_case_globals)]
pub const ldelem_u1: &'static [u8] = &[145];
#[allow(non_upper_case_globals)]
pub const ldelem_u2: &'static [u8] = &[147];
#[allow(non_upper_case_globals)]
pub const ldelem_u4: &'static [u8] = &[149];
#[allow(non_upper_case_globals)]
pub const ldelem_u8: &'static [u8] = &[150];
#[allow(non_upper_case_globals)]
pub const ldelema: &'static [u8] = &[143];
#[allow(non_upper_case_globals)]
pub const ldfld: &'static [u8] = &[123];
#[allow(non_upper_case_globals)]
pub const ldflda: &'static [u8] = &[124];
#[allow(non_upper_case_globals)]
pub const ldftn: &'static [u8] = &[254, 6];
#[allow(non_upper_case_globals)]
pub const ldind_i: &'static [u8] = &[77];
#[allow(non_upper_case_globals)]
pub const ldind_i1: &'static [u8] = &[70];
#[allow(non_upper_case_globals)]
pub const ldind_i2: &'static [u8] = &[72];
#[allow(non_upper_case_globals)]
pub const ldind_i4: &'static [u8] = &[74];
#[allow(non_upper_case_globals)]
pub const ldind_i8: &'static [u8] = &[76];
#[allow(non_upper_case_globals)]
pub const ldind_r4: &'static [u8] = &[78];
#[allow(non_upper_case_globals)]
pub const ldind_r8: &'static [u8] = &[79];
#[allow(non_upper_case_globals)]
pub const ldind_ref: &'static [u8] = &[80];
#[allow(non_upper_case_globals)]
pub const ldind_u1: &'static [u8] = &[71];
#[allow(non_upper_case_globals)]
pub const ldind_u2: &'static [u8] = &[73];
#[allow(non_upper_case_globals)]
pub const ldind_u4: &'static [u8] = &[75];
#[allow(non_upper_case_globals)]
pub const ldind_u8: &'static [u8] = &[76];
#[allow(non_upper_case_globals)]
pub const ldlen: &'static [u8] = &[142];
#[allow(non_upper_case_globals)]
pub const ldloc_0: &'static [u8] = &[6];
#[allow(non_upper_case_globals)]
pub const ldloc_1: &'static [u8] = &[7];
#[allow(non_upper_case_globals)]
pub const ldloc_2: &'static [u8] = &[8];
#[allow(non_upper_case_globals)]
pub const ldloc_3: &'static [u8] = &[9];
#[allow(non_upper_case_globals)]
pub const ldloc_s: &'static [u8] = &[17];
#[allow(non_upper_case_globals)]
pub const ldloca: &'static [u8] = &[254, 13];
#[allow(non_upper_case_globals)]
pub const ldloca_s: &'static [u8] = &[18];
#[allow(non_upper_case_globals)]
pub const ldnull: &'static [u8] = &[20];
#[allow(non_upper_case_globals)]
pub const ldobj: &'static [u8] = &[113];
#[allow(non_upper_case_globals)]
pub const ldsfld: &'static [u8] = &[126];
#[allow(non_upper_case_globals)]
pub const ldsflda: &'static [u8] = &[127];
#[allow(non_upper_case_globals)]
pub const ldstr: &'static [u8] = &[114];
#[allow(non_upper_case_globals)]
pub const ldtoken: &'static [u8] = &[208];
#[allow(non_upper_case_globals)]
pub const ldvirtfn: &'static [u8] = &[254, 7];
#[allow(non_upper_case_globals)]
pub const leave: &'static [u8] = &[221];
#[allow(non_upper_case_globals)]
pub const leave_s: &'static [u8] = &[222];
#[allow(non_upper_case_globals)]
pub const localloc: &'static [u8] = &[254, 15];
#[allow(non_upper_case_globals)]
pub const mkrefany: &'static [u8] = &[198];
#[allow(non_upper_case_globals)]
pub const mul: &'static [u8] = &[90];
#[allow(non_upper_case_globals)]
pub const mul_ovf: &'static [u8] = &[216];
#[allow(non_upper_case_globals)]
pub const mul_ovf_un: &'static [u8] = &[217];
#[allow(non_upper_case_globals)]
pub const neg: &'static [u8] = &[101];
#[allow(non_upper_case_globals)]
pub const newarr: &'static [u8] = &[141];
#[allow(non_upper_case_globals)]
pub const newobj: &'static [u8] = &[115];
#[allow(non_upper_case_globals)]
pub const no_: &'static [u8] = &[254, 25];
#[allow(non_upper_case_globals)]
pub const nop: &'static [u8] = &[0];
#[allow(non_upper_case_globals)]
pub const not: &'static [u8] = &[102];
#[allow(non_upper_case_globals)]
pub const or: &'static [u8] = &[96];
#[allow(non_upper_case_globals)]
pub const pop: &'static [u8] = &[38];
#[allow(non_upper_case_globals)]
pub const readonly: &'static [u8] = &[254, 30];
#[allow(non_upper_case_globals)]
pub const refanytype: &'static [u8] = &[254, 29];
#[allow(non_upper_case_globals)]
pub const refanyval: &'static [u8] = &[194];
#[allow(non_upper_case_globals)]
pub const rem: &'static [u8] = &[93];
#[allow(non_upper_case_globals)]
pub const rem_un: &'static [u8] = &[94];
#[allow(non_upper_case_globals)]
pub const ret: &'static [u8] = &[42];
#[allow(non_upper_case_globals)]
pub const rethrow: &'static [u8] = &[254, 26];
#[allow(non_upper_case_globals)]
pub const shl: &'static [u8] = &[98];
#[allow(non_upper_case_globals)]
pub const shr: &'static [u8] = &[99];
#[allow(non_upper_case_globals)]
pub const shr_un: &'static [u8] = &[100];
#[allow(non_upper_case_globals)]
pub const sizeof: &'static [u8] = &[254, 28];
#[allow(non_upper_case_globals)]
pub const starg: &'static [u8] = &[254, 11];
#[allow(non_upper_case_globals)]
pub const starg_s: &'static [u8] = &[16];
#[allow(non_upper_case_globals)]
pub const stelem: &'static [u8] = &[164];
#[allow(non_upper_case_globals)]
pub const stelem_i: &'static [u8] = &[155];
#[allow(non_upper_case_globals)]
pub const stelem_i1: &'static [u8] = &[156];
#[allow(non_upper_case_globals)]
pub const stelem_i2: &'static [u8] = &[157];
#[allow(non_upper_case_globals)]
pub const stelem_i4: &'static [u8] = &[158];
#[allow(non_upper_case_globals)]
pub const stelem_i8: &'static [u8] = &[159];
#[allow(non_upper_case_globals)]
pub const stelem_r4: &'static [u8] = &[160];
#[allow(non_upper_case_globals)]
pub const stelem_r8: &'static [u8] = &[161];
#[allow(non_upper_case_globals)]
pub const stelem_ref: &'static [u8] = &[162];
#[allow(non_upper_case_globals)]
pub const stfld: &'static [u8] = &[125];
#[allow(non_upper_case_globals)]
pub const stind_i: &'static [u8] = &[223];
#[allow(non_upper_case_globals)]
pub const stind_i1: &'static [u8] = &[82];
#[allow(non_upper_case_globals)]
pub const stind_i2: &'static [u8] = &[83];
#[allow(non_upper_case_globals)]
pub const stind_i4: &'static [u8] = &[84];
#[allow(non_upper_case_globals)]
pub const stind_i8: &'static [u8] = &[85];
#[allow(non_upper_case_globals)]
pub const stind_r4: &'static [u8] = &[86];
#[allow(non_upper_case_globals)]
pub const stind_r8: &'static [u8] = &[87];
#[allow(non_upper_case_globals)]
pub const stind_ref: &'static [u8] = &[81];
#[allow(non_upper_case_globals)]
pub const stloc: &'static [u8] = &[254, 14];
#[allow(non_upper_case_globals)]
pub const stloc_0: &'static [u8] = &[10];
#[allow(non_upper_case_globals)]
pub const stloc_1: &'static [u8] = &[11];
#[allow(non_upper_case_globals)]
pub const stloc_2: &'static [u8] = &[12];
#[allow(non_upper_case_globals)]
pub const stloc_3: &'static [u8] = &[13];
#[allow(non_upper_case_globals)]
pub const stloc_s: &'static [u8] = &[19];
#[allow(non_upper_case_globals)]
pub const stobj: &'static [u8] = &[129];
#[allow(non_upper_case_globals)]
pub const stsfld: &'static [u8] = &[128];
#[allow(non_upper_case_globals)]
pub const sub: &'static [u8] = &[89];
#[allow(non_upper_case_globals)]
pub const sub_ovf: &'static [u8] = &[218];
#[allow(non_upper_case_globals)]
pub const sub_ovf_un: &'static [u8] = &[219];
#[allow(non_upper_case_globals)]
pub const switch: &'static [u8] = &[69];
#[allow(non_upper_case_globals)]
pub const tailcall: &'static [u8] = &[254, 20];
#[allow(non_upper_case_globals)]
pub const throw: &'static [u8] = &[122];
#[allow(non_upper_case_globals)]
pub const unaligned: &'static [u8] = &[254, 18];
#[allow(non_upper_case_globals)]
pub const unbox: &'static [u8] = &[121];
#[allow(non_upper_case_globals)]
pub const unbox_any: &'static [u8] = &[165];
#[allow(non_upper_case_globals)]
pub const volatile: &'static [u8] = &[254, 19];
#[allow(non_upper_case_globals)]
pub const xor: &'static [u8] = &[97];
#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub struct OPCODE_TABLE {
    __private_field: (),
}
#[doc(hidden)]
pub static OPCODE_TABLE: OPCODE_TABLE = OPCODE_TABLE {
    __private_field: (),
};
impl ::lazy_static::__Deref for OPCODE_TABLE {
    type Target = HashMap<Vec<u8>, String>;
    fn deref(&self) -> &HashMap<Vec<u8>, String> {
        #[inline(always)]
        fn __static_ref_initialize() -> HashMap<Vec<u8>, String> {
            {
                let mut map = HashMap::new();
                map.insert(
                    [88].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "add".to_string(),
                );
                map.insert(
                    [214].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "add.ovf".to_string(),
                );
                map.insert(
                    [215].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "add.ovf.un".to_string(),
                );
                map.insert(
                    [95].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "and".to_string(),
                );
                map.insert(
                    [254, 0].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "arglist".to_string(),
                );
                map.insert(
                    [59].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "beq".to_string(),
                );
                map.insert(
                    [46].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "beq.s".to_string(),
                );
                map.insert(
                    [60].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "bge".to_string(),
                );
                map.insert(
                    [242].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "bge.s".to_string(),
                );
                map.insert(
                    [65].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "bge.un".to_string(),
                );
                map.insert(
                    [52].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "bge.un.s".to_string(),
                );
                map.insert(
                    [61].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "bgt".to_string(),
                );
                map.insert(
                    [48].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "bgt.s".to_string(),
                );
                map.insert(
                    [66].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "bgt.un".to_string(),
                );
                map.insert(
                    [53].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "bgt.un.s".to_string(),
                );
                map.insert(
                    [62].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ble".to_string(),
                );
                map.insert(
                    [49].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ble.s".to_string(),
                );
                map.insert(
                    [67].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ble.un".to_string(),
                );
                map.insert(
                    [54].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ble.un.s".to_string(),
                );
                map.insert(
                    [63].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "blt".to_string(),
                );
                map.insert(
                    [50].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "blt.s".to_string(),
                );
                map.insert(
                    [68].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "blt.un".to_string(),
                );
                map.insert(
                    [55].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "blt.un.s".to_string(),
                );
                map.insert(
                    [64].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "bne.un".to_string(),
                );
                map.insert(
                    [51].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "bne.un.s".to_string(),
                );
                map.insert(
                    [140].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "box".to_string(),
                );
                map.insert(
                    [56].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "br".to_string(),
                );
                map.insert(
                    [43].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "bs.s".to_string(),
                );
                map.insert(
                    [1].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "break".to_string(),
                );
                map.insert(
                    [57].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "brfalse".to_string(),
                );
                map.insert(
                    [44].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "brfalse.s".to_string(),
                );
                map.insert(
                    [58].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "brinst".to_string(),
                );
                map.insert(
                    [45].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "brinst.s".to_string(),
                );
                map.insert(
                    [57].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "brnull".to_string(),
                );
                map.insert(
                    [44].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "brnull.s".to_string(),
                );
                map.insert(
                    [58].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "brtrue".to_string(),
                );
                map.insert(
                    [45].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "brtrue.s".to_string(),
                );
                map.insert(
                    [57].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "brzero".to_string(),
                );
                map.insert(
                    [44].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "brzero.s".to_string(),
                );
                map.insert(
                    [40].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "call".to_string(),
                );
                map.insert(
                    [41].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "calli".to_string(),
                );
                map.insert(
                    [111].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "callvirt".to_string(),
                );
                map.insert(
                    [116].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "castclass".to_string(),
                );
                map.insert(
                    [254, 1].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ceq".to_string(),
                );
                map.insert(
                    [254, 2].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "cgt".to_string(),
                );
                map.insert(
                    [254, 3].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "cgt.un".to_string(),
                );
                map.insert(
                    [195].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ckfinite".to_string(),
                );
                map.insert(
                    [254, 4].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "clt".to_string(),
                );
                map.insert(
                    [254, 5].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "clt.un".to_string(),
                );
                map.insert(
                    [254, 22].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "constrained".to_string(),
                );
                map.insert(
                    [211].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.i".to_string(),
                );
                map.insert(
                    [103].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.i1".to_string(),
                );
                map.insert(
                    [104].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.i2".to_string(),
                );
                map.insert(
                    [105].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.i4".to_string(),
                );
                map.insert(
                    [106].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.i8".to_string(),
                );
                map.insert(
                    [212].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.i".to_string(),
                );
                map.insert(
                    [138].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.i.un".to_string(),
                );
                map.insert(
                    [179].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.i1".to_string(),
                );
                map.insert(
                    [130].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.i1.un".to_string(),
                );
                map.insert(
                    [181].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.i2".to_string(),
                );
                map.insert(
                    [131].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.i2.un".to_string(),
                );
                map.insert(
                    [183].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.i4".to_string(),
                );
                map.insert(
                    [132].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.i4.un".to_string(),
                );
                map.insert(
                    [185].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.i8".to_string(),
                );
                map.insert(
                    [133].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "convf.ovf.i8.un".to_string(),
                );
                map.insert(
                    [213].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "convf.ovf.u".to_string(),
                );
                map.insert(
                    [139].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.u.un".to_string(),
                );
                map.insert(
                    [180].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.u1".to_string(),
                );
                map.insert(
                    [134].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.u1.un".to_string(),
                );
                map.insert(
                    [182].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.u2".to_string(),
                );
                map.insert(
                    [135].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.u2.un".to_string(),
                );
                map.insert(
                    [184].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.u4".to_string(),
                );
                map.insert(
                    [136].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "convf.ovf.u4.un".to_string(),
                );
                map.insert(
                    [186].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.u8".to_string(),
                );
                map.insert(
                    [137].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.ovf.u8.un".to_string(),
                );
                map.insert(
                    [118].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.r.un".to_string(),
                );
                map.insert(
                    [107].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.r4".to_string(),
                );
                map.insert(
                    [108].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.r8".to_string(),
                );
                map.insert(
                    [224].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.u".to_string(),
                );
                map.insert(
                    [210].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.u1".to_string(),
                );
                map.insert(
                    [209].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.u2".to_string(),
                );
                map.insert(
                    [109].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.u4".to_string(),
                );
                map.insert(
                    [110].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "conv.u8".to_string(),
                );
                map.insert(
                    [254, 23].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "cpblk".to_string(),
                );
                map.insert(
                    [112].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "cpobj".to_string(),
                );
                map.insert(
                    [91].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "div".to_string(),
                );
                map.insert(
                    [92].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "div.un".to_string(),
                );
                map.insert(
                    [37].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "dup".to_string(),
                );
                map.insert(
                    [220].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "endfault".to_string(),
                );
                map.insert(
                    [254, 17].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "endfilter".to_string(),
                );
                map.insert(
                    [220].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "endfinally".to_string(),
                );
                map.insert(
                    [254, 24].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "initblk".to_string(),
                );
                map.insert(
                    [254, 21].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "initobj".to_string(),
                );
                map.insert(
                    [117].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "isinst".to_string(),
                );
                map.insert(
                    [39].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "jmp".to_string(),
                );
                map.insert(
                    [254, 9].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldarg".to_string(),
                );
                map.insert(
                    [2].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldarg.0".to_string(),
                );
                map.insert(
                    [3].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldarg.1".to_string(),
                );
                map.insert(
                    [4].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldarg.2".to_string(),
                );
                map.insert(
                    [5].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldarg.3".to_string(),
                );
                map.insert(
                    [14].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldarg.s".to_string(),
                );
                map.insert(
                    [254, 10].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldarga".to_string(),
                );
                map.insert(
                    [15].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldarga.s".to_string(),
                );
                map.insert(
                    [32].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.i4".to_string(),
                );
                map.insert(
                    [22].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.i4.0".to_string(),
                );
                map.insert(
                    [23].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.i4.1".to_string(),
                );
                map.insert(
                    [24].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.i4.2".to_string(),
                );
                map.insert(
                    [25].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.i4.3".to_string(),
                );
                map.insert(
                    [26].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.i4.4".to_string(),
                );
                map.insert(
                    [27].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.i4.5".to_string(),
                );
                map.insert(
                    [28].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.i4.6".to_string(),
                );
                map.insert(
                    [29].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.i4.7".to_string(),
                );
                map.insert(
                    [30].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.i4.8".to_string(),
                );
                map.insert(
                    [21].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.i4.m1".to_string(),
                );
                map.insert(
                    [31].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.i4.s".to_string(),
                );
                map.insert(
                    [33].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.i8".to_string(),
                );
                map.insert(
                    [34].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.r4".to_string(),
                );
                map.insert(
                    [35].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldc.r8".to_string(),
                );
                map.insert(
                    [163].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldelem".to_string(),
                );
                map.insert(
                    [151].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldelem.i".to_string(),
                );
                map.insert(
                    [144].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldelem.i1".to_string(),
                );
                map.insert(
                    [146].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldelem.i2".to_string(),
                );
                map.insert(
                    [148].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldelem.i4".to_string(),
                );
                map.insert(
                    [150].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldelem.i8".to_string(),
                );
                map.insert(
                    [152].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldelem.r4".to_string(),
                );
                map.insert(
                    [153].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldelem.r8".to_string(),
                );
                map.insert(
                    [154].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldelem.ref".to_string(),
                );
                map.insert(
                    [145].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldelem.u1".to_string(),
                );
                map.insert(
                    [147].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldelem.u2".to_string(),
                );
                map.insert(
                    [149].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldelem.u4".to_string(),
                );
                map.insert(
                    [150].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldelem.u8".to_string(),
                );
                map.insert(
                    [143].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldelema".to_string(),
                );
                map.insert(
                    [123].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldfld".to_string(),
                );
                map.insert(
                    [124].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldflda".to_string(),
                );
                map.insert(
                    [254, 6].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldftn".to_string(),
                );
                map.insert(
                    [77].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldind.i".to_string(),
                );
                map.insert(
                    [70].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldind.i1".to_string(),
                );
                map.insert(
                    [72].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldind.i2".to_string(),
                );
                map.insert(
                    [74].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldind.i4".to_string(),
                );
                map.insert(
                    [76].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldind.i8".to_string(),
                );
                map.insert(
                    [78].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldind.r4".to_string(),
                );
                map.insert(
                    [79].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldind.r8".to_string(),
                );
                map.insert(
                    [80].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldind.ref".to_string(),
                );
                map.insert(
                    [71].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldind.u1".to_string(),
                );
                map.insert(
                    [73].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldind.u2".to_string(),
                );
                map.insert(
                    [75].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldind.u4".to_string(),
                );
                map.insert(
                    [76].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldind.i8".to_string(),
                );
                map.insert(
                    [142].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldlen".to_string(),
                );
                map.insert(
                    [6].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldloc.0".to_string(),
                );
                map.insert(
                    [7].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldloc.1".to_string(),
                );
                map.insert(
                    [8].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldloc.2".to_string(),
                );
                map.insert(
                    [9].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldloc.3".to_string(),
                );
                map.insert(
                    [17].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldloc.s".to_string(),
                );
                map.insert(
                    [254, 13].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldloca".to_string(),
                );
                map.insert(
                    [18].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldloca.s".to_string(),
                );
                map.insert(
                    [20].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldnull".to_string(),
                );
                map.insert(
                    [113].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldobj".to_string(),
                );
                map.insert(
                    [126].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldsfld".to_string(),
                );
                map.insert(
                    [127].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldsflda".to_string(),
                );
                map.insert(
                    [114].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldstr".to_string(),
                );
                map.insert(
                    [208].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldtoken".to_string(),
                );
                map.insert(
                    [254, 7].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ldvirtfn".to_string(),
                );
                map.insert(
                    [221].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "leave".to_string(),
                );
                map.insert(
                    [222].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "leave.s".to_string(),
                );
                map.insert(
                    [254, 15].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "localloc".to_string(),
                );
                map.insert(
                    [198].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "mkrefany".to_string(),
                );
                map.insert(
                    [90].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "mul".to_string(),
                );
                map.insert(
                    [216].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "mul.ovf".to_string(),
                );
                map.insert(
                    [217].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "mul.ovf.un".to_string(),
                );
                map.insert(
                    [101].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "neg".to_string(),
                );
                map.insert(
                    [141].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "newarr".to_string(),
                );
                map.insert(
                    [115].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "newobj".to_string(),
                );
                map.insert(
                    [254, 25].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "no. {typecheck,rangecheck,nullcheck}".to_string(),
                );
                map.insert(
                    [0].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "nop".to_string(),
                );
                map.insert(
                    [102].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "not".to_string(),
                );
                map.insert(
                    [96].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "or".to_string(),
                );
                map.insert(
                    [38].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "pop".to_string(),
                );
                map.insert(
                    [254, 30].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "readonly".to_string(),
                );
                map.insert(
                    [254, 29].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "refanytype".to_string(),
                );
                map.insert(
                    [194].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "refanyval".to_string(),
                );
                map.insert(
                    [93].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "rem".to_string(),
                );
                map.insert(
                    [94].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "rem.un".to_string(),
                );
                map.insert(
                    [42].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "ret".to_string(),
                );
                map.insert(
                    [254, 26].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "rethrow".to_string(),
                );
                map.insert(
                    [98].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "shl".to_string(),
                );
                map.insert(
                    [99].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "shr".to_string(),
                );
                map.insert(
                    [100].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "shr.un".to_string(),
                );
                map.insert(
                    [254, 28].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "sizeof".to_string(),
                );
                map.insert(
                    [254, 11].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "starg".to_string(),
                );
                map.insert(
                    [16].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "starg.s".to_string(),
                );
                map.insert(
                    [164].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stelem".to_string(),
                );
                map.insert(
                    [155].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stelem.i".to_string(),
                );
                map.insert(
                    [156].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stelem.i1".to_string(),
                );
                map.insert(
                    [157].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stelem.i2".to_string(),
                );
                map.insert(
                    [158].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stelem.i4".to_string(),
                );
                map.insert(
                    [159].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stelem.i8".to_string(),
                );
                map.insert(
                    [160].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stelem.r4".to_string(),
                );
                map.insert(
                    [161].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stelem.r8".to_string(),
                );
                map.insert(
                    [162].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stelem.ref".to_string(),
                );
                map.insert(
                    [125].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stfld".to_string(),
                );
                map.insert(
                    [223].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stind.i".to_string(),
                );
                map.insert(
                    [82].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stind.i1".to_string(),
                );
                map.insert(
                    [83].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stind.i2".to_string(),
                );
                map.insert(
                    [84].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stind.i4".to_string(),
                );
                map.insert(
                    [85].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stind.i8".to_string(),
                );
                map.insert(
                    [86].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stind.r4".to_string(),
                );
                map.insert(
                    [87].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stind.r8".to_string(),
                );
                map.insert(
                    [81].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stind.ref".to_string(),
                );
                map.insert(
                    [254, 14].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stloc".to_string(),
                );
                map.insert(
                    [10].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stloc.0".to_string(),
                );
                map.insert(
                    [11].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stloc.1".to_string(),
                );
                map.insert(
                    [12].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stloc.2".to_string(),
                );
                map.insert(
                    [13].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stloc.3".to_string(),
                );
                map.insert(
                    [19].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stloc.s".to_string(),
                );
                map.insert(
                    [129].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stobj".to_string(),
                );
                map.insert(
                    [128].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "stsfld".to_string(),
                );
                map.insert(
                    [89].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "sub".to_string(),
                );
                map.insert(
                    [218].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "sub.ovf".to_string(),
                );
                map.insert(
                    [219].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "sub.ovf.un".to_string(),
                );
                map.insert(
                    [69].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "switch".to_string(),
                );
                map.insert(
                    [254, 20].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "tail".to_string(),
                );
                map.insert(
                    [122].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "throw".to_string(),
                );
                map.insert(
                    [254, 18].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "unaligned".to_string(),
                );
                map.insert(
                    [121].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "unbox".to_string(),
                );
                map.insert(
                    [165].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "unbox.any".to_string(),
                );
                map.insert(
                    [254, 19].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "volatile".to_string(),
                );
                map.insert(
                    [97].iter().map(|b| *b).collect::<Vec<u8>>(),
                    "xor".to_string(),
                );
                map
            }
        }
        #[inline(always)]
        fn __stability() -> &'static HashMap<Vec<u8>, String> {
            static LAZY: ::lazy_static::lazy::Lazy<HashMap<Vec<u8>, String>> =
                ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for OPCODE_TABLE {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub struct OPCODE_TO_BYTES {
    __private_field: (),
}
#[doc(hidden)]
pub static OPCODE_TO_BYTES: OPCODE_TO_BYTES = OPCODE_TO_BYTES {
    __private_field: (),
};
impl ::lazy_static::__Deref for OPCODE_TO_BYTES {
    type Target = HashMap<String, Vec<u8>>;
    fn deref(&self) -> &HashMap<String, Vec<u8>> {
        #[inline(always)]
        fn __static_ref_initialize() -> HashMap<String, Vec<u8>> {
            {
                let mut map = HashMap::new();
                map.insert(
                    "add".to_string(),
                    [88].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "add.ovf".to_string(),
                    [214].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "add.ovf.un".to_string(),
                    [215].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "and".to_string(),
                    [95].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "arglist".to_string(),
                    [254, 0].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "beq".to_string(),
                    [59].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "beq.s".to_string(),
                    [46].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "bge".to_string(),
                    [60].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "bge.s".to_string(),
                    [242].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "bge.un".to_string(),
                    [65].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "bge.un.s".to_string(),
                    [52].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "bgt".to_string(),
                    [61].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "bgt.s".to_string(),
                    [48].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "bgt.un".to_string(),
                    [66].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "bgt.un.s".to_string(),
                    [53].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ble".to_string(),
                    [62].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ble.s".to_string(),
                    [49].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ble.un".to_string(),
                    [67].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ble.un.s".to_string(),
                    [54].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "blt".to_string(),
                    [63].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "blt.s".to_string(),
                    [50].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "blt.un".to_string(),
                    [68].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "blt.un.s".to_string(),
                    [55].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "bne.un".to_string(),
                    [64].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "bne.un.s".to_string(),
                    [51].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "box".to_string(),
                    [140].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "br".to_string(),
                    [56].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "bs.s".to_string(),
                    [43].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "break".to_string(),
                    [1].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "brfalse".to_string(),
                    [57].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "brfalse.s".to_string(),
                    [44].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "brinst".to_string(),
                    [58].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "brinst.s".to_string(),
                    [45].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "brnull".to_string(),
                    [57].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "brnull.s".to_string(),
                    [44].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "brtrue".to_string(),
                    [58].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "brtrue.s".to_string(),
                    [45].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "brzero".to_string(),
                    [57].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "brzero.s".to_string(),
                    [44].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "call".to_string(),
                    [40].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "calli".to_string(),
                    [41].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "callvirt".to_string(),
                    [111].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "castclass".to_string(),
                    [116].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ceq".to_string(),
                    [254, 1].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "cgt".to_string(),
                    [254, 2].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "cgt.un".to_string(),
                    [254, 3].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ckfinite".to_string(),
                    [195].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "clt".to_string(),
                    [254, 4].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "clt.un".to_string(),
                    [254, 5].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "constrained".to_string(),
                    [254, 22].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.i".to_string(),
                    [211].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.i1".to_string(),
                    [103].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.i2".to_string(),
                    [104].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.i4".to_string(),
                    [105].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.i8".to_string(),
                    [106].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.i".to_string(),
                    [212].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.i.un".to_string(),
                    [138].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.i1".to_string(),
                    [179].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.i1.un".to_string(),
                    [130].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.i2".to_string(),
                    [181].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.i2.un".to_string(),
                    [131].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.i4".to_string(),
                    [183].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.i4.un".to_string(),
                    [132].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.i8".to_string(),
                    [185].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "convf.ovf.i8.un".to_string(),
                    [133].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "convf.ovf.u".to_string(),
                    [213].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.u.un".to_string(),
                    [139].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.u1".to_string(),
                    [180].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.u1.un".to_string(),
                    [134].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.u2".to_string(),
                    [182].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.u2.un".to_string(),
                    [135].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.u4".to_string(),
                    [184].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "convf.ovf.u4.un".to_string(),
                    [136].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.u8".to_string(),
                    [186].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.ovf.u8.un".to_string(),
                    [137].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.r.un".to_string(),
                    [118].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.r4".to_string(),
                    [107].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.r8".to_string(),
                    [108].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.u".to_string(),
                    [224].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.u1".to_string(),
                    [210].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.u2".to_string(),
                    [209].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.u4".to_string(),
                    [109].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "conv.u8".to_string(),
                    [110].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "cpblk".to_string(),
                    [254, 23].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "cpobj".to_string(),
                    [112].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "div".to_string(),
                    [91].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "div.un".to_string(),
                    [92].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "dup".to_string(),
                    [37].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "endfault".to_string(),
                    [220].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "endfilter".to_string(),
                    [254, 17].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "endfinally".to_string(),
                    [220].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "initblk".to_string(),
                    [254, 24].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "initobj".to_string(),
                    [254, 21].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "isinst".to_string(),
                    [117].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "jmp".to_string(),
                    [39].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldarg".to_string(),
                    [254, 9].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldarg.0".to_string(),
                    [2].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldarg.1".to_string(),
                    [3].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldarg.2".to_string(),
                    [4].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldarg.3".to_string(),
                    [5].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldarg.s".to_string(),
                    [14].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldarga".to_string(),
                    [254, 10].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldarga.s".to_string(),
                    [15].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.i4".to_string(),
                    [32].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.i4.0".to_string(),
                    [22].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.i4.1".to_string(),
                    [23].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.i4.2".to_string(),
                    [24].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.i4.3".to_string(),
                    [25].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.i4.4".to_string(),
                    [26].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.i4.5".to_string(),
                    [27].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.i4.6".to_string(),
                    [28].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.i4.7".to_string(),
                    [29].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.i4.8".to_string(),
                    [30].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.i4.m1".to_string(),
                    [21].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.i4.s".to_string(),
                    [31].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.i8".to_string(),
                    [33].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.r4".to_string(),
                    [34].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldc.r8".to_string(),
                    [35].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldelem".to_string(),
                    [163].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldelem.i".to_string(),
                    [151].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldelem.i1".to_string(),
                    [144].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldelem.i2".to_string(),
                    [146].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldelem.i4".to_string(),
                    [148].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldelem.i8".to_string(),
                    [150].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldelem.r4".to_string(),
                    [152].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldelem.r8".to_string(),
                    [153].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldelem.ref".to_string(),
                    [154].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldelem.u1".to_string(),
                    [145].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldelem.u2".to_string(),
                    [147].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldelem.u4".to_string(),
                    [149].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldelem.u8".to_string(),
                    [150].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldelema".to_string(),
                    [143].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldfld".to_string(),
                    [123].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldflda".to_string(),
                    [124].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldftn".to_string(),
                    [254, 6].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldind.i".to_string(),
                    [77].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldind.i1".to_string(),
                    [70].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldind.i2".to_string(),
                    [72].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldind.i4".to_string(),
                    [74].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldind.i8".to_string(),
                    [76].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldind.r4".to_string(),
                    [78].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldind.r8".to_string(),
                    [79].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldind.ref".to_string(),
                    [80].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldind.u1".to_string(),
                    [71].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldind.u2".to_string(),
                    [73].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldind.u4".to_string(),
                    [75].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldind.i8".to_string(),
                    [76].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldlen".to_string(),
                    [142].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldloc.0".to_string(),
                    [6].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldloc.1".to_string(),
                    [7].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldloc.2".to_string(),
                    [8].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldloc.3".to_string(),
                    [9].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldloc.s".to_string(),
                    [17].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldloca".to_string(),
                    [254, 13].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldloca.s".to_string(),
                    [18].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldnull".to_string(),
                    [20].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldobj".to_string(),
                    [113].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldsfld".to_string(),
                    [126].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldsflda".to_string(),
                    [127].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldstr".to_string(),
                    [114].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldtoken".to_string(),
                    [208].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ldvirtfn".to_string(),
                    [254, 7].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "leave".to_string(),
                    [221].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "leave.s".to_string(),
                    [222].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "localloc".to_string(),
                    [254, 15].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "mkrefany".to_string(),
                    [198].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "mul".to_string(),
                    [90].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "mul.ovf".to_string(),
                    [216].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "mul.ovf.un".to_string(),
                    [217].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "neg".to_string(),
                    [101].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "newarr".to_string(),
                    [141].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "newobj".to_string(),
                    [115].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "no. {typecheck,rangecheck,nullcheck}".to_string(),
                    [254, 25].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "nop".to_string(),
                    [0].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "not".to_string(),
                    [102].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "or".to_string(),
                    [96].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "pop".to_string(),
                    [38].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "readonly".to_string(),
                    [254, 30].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "refanytype".to_string(),
                    [254, 29].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "refanyval".to_string(),
                    [194].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "rem".to_string(),
                    [93].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "rem.un".to_string(),
                    [94].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "ret".to_string(),
                    [42].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "rethrow".to_string(),
                    [254, 26].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "shl".to_string(),
                    [98].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "shr".to_string(),
                    [99].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "shr.un".to_string(),
                    [100].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "sizeof".to_string(),
                    [254, 28].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "starg".to_string(),
                    [254, 11].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "starg.s".to_string(),
                    [16].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stelem".to_string(),
                    [164].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stelem.i".to_string(),
                    [155].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stelem.i1".to_string(),
                    [156].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stelem.i2".to_string(),
                    [157].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stelem.i4".to_string(),
                    [158].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stelem.i8".to_string(),
                    [159].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stelem.r4".to_string(),
                    [160].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stelem.r8".to_string(),
                    [161].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stelem.ref".to_string(),
                    [162].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stfld".to_string(),
                    [125].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stind.i".to_string(),
                    [223].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stind.i1".to_string(),
                    [82].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stind.i2".to_string(),
                    [83].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stind.i4".to_string(),
                    [84].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stind.i8".to_string(),
                    [85].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stind.r4".to_string(),
                    [86].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stind.r8".to_string(),
                    [87].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stind.ref".to_string(),
                    [81].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stloc".to_string(),
                    [254, 14].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stloc.0".to_string(),
                    [10].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stloc.1".to_string(),
                    [11].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stloc.2".to_string(),
                    [12].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stloc.3".to_string(),
                    [13].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stloc.s".to_string(),
                    [19].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stobj".to_string(),
                    [129].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "stsfld".to_string(),
                    [128].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "sub".to_string(),
                    [89].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "sub.ovf".to_string(),
                    [218].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "sub.ovf.un".to_string(),
                    [219].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "switch".to_string(),
                    [69].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "tail".to_string(),
                    [254, 20].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "throw".to_string(),
                    [122].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "unaligned".to_string(),
                    [254, 18].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "unbox".to_string(),
                    [121].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "unbox.any".to_string(),
                    [165].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "volatile".to_string(),
                    [254, 19].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map.insert(
                    "xor".to_string(),
                    [97].iter().map(|b| *b).collect::<Vec<u8>>(),
                );
                map
            }
        }
        #[inline(always)]
        fn __stability() -> &'static HashMap<String, Vec<u8>> {
            static LAZY: ::lazy_static::lazy::Lazy<HashMap<String, Vec<u8>>> =
                ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for OPCODE_TO_BYTES {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
