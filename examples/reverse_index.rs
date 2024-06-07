use bitcoin_script_stack::stack::StackTracker;
#[cfg(feature = "interactive")]
use bitcoin_script_stack::interactive::interactive;

#[allow(dead_code)]
fn reverse_index() -> StackTracker {

    let mut stack = StackTracker::new();
    stack.number(1);
    stack.number(10);
    stack.number(5);
    stack.number(3);

    stack.op_equalverify();
    stack.op_add();
    stack.to_altstack();
    stack.to_altstack();
    stack.from_altstack();
    stack.from_altstack();
    stack.op_2drop();
    stack
}


fn main() {
    let stack = reverse_index();
    println!("Stack: {:?}", stack.get_script());
    println!("Scipt Len: {:?}", stack.get_script_len());

    
    #[cfg(feature = "interactive")] 
    {
        interactive(&example(false));
        interactive(&example(true));
    }
    #[cfg(not(feature = "interactive"))]
    println!("Executed with --features interactive");
}
