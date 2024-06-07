use bitcoin_script::{define_pushable, script};
use bitcoin_script_stack::stack::StackTracker;

define_pushable!();

pub fn main() {
    let mut stack = StackTracker::new();        
                                            
    let var1 = stack.number(1);        
    let var2 = stack.number(10);        
    stack.debug();     
    let _ = stack.copy_var(var1);   
    
    stack.debug();

    stack.move_var(var2);
    stack.drop(var2);      
    stack.op_equalverify();
    stack.op_true();     

    let double_script  = script!{
        OP_DUP
        OP_ADD
    };
    let double_output = stack.custom(double_script, 1, true, 0, "double").unwrap();

    stack.debug();

    stack.number(2);
    stack.op_equalverify();
    stack.op_true();     



}