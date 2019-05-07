use rustc::mir::*;
use rustc::ty::*;

use std::collections::HashMap;

pub struct Transform<'a,'tcx: 'a> {
    mir: &'a mut Mir<'tcx>,
    locals: HashMap<Local,u32>,

}

impl<'a,'tcx: 'a> Transform<'a,'tcx> {
    fn statement(&mut self,s: &'a Statement<'tcx>) {
        match &s.kind {
            StatementKind::InlineAsm(_) => {
                eprintln!("Inline asm not supported in IronOxide");
            }
            _ => unimplemented!()
        }
    }

    fn rvalue(&mut self,r: &'a Rvalue<'tcx>) {
        match &r {
            // emit conv.<ty>
            Rvalue::Cast(kind,operand,ty) => {
                
            }
            // emit <op> or <op>.un
            Rvalue::BinaryOp(binop,op1,op2) => {

            }
            // emit <op>.ovf or <op>.ovf.un
            Rvalue::CheckedBinaryOp(binop,op1,op2) => {

            }
            // emit mkrefany <type>
            Rvalue::Ref(reg,kind,place) => {

            }
            // emit ldlen
            Rvalue::Len(place) => {

            }
            // emit newarr and then stelem.<elem_ty> <idx>
            Rvalue::Repeat(operand,count) => {

            }
            // emit box or sizeof
            // TODO: Check type is boxable, if boxable emit box otherwise just emit value since CLR got GC
            Rvalue::NullaryOp(op,ty) => {

            }
            // emit newobj or newarr if kind is tuple
            Rvalue::Aggregate(kind,operands) => {

            }   
            // just emit this operand?
            Rvalue::Use(operand) => {

            }
            // emit <op>
            Rvalue::UnaryOp(op,operand) => {

            }
        
            Rvalue::Discriminant(place) => {

            }

        }
    }
}