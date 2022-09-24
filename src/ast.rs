#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Node {
    Num(i32),
    BinOp {
        op: Operator,
        left: Box<Node>,
        right: Box<Node>
    }
}