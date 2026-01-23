//! SSL 4.0 Formal Verification
//!
//! Pre/Post-conditions, invariants, and formal proofs.

use std::collections::HashMap;

/// Contract specification
#[derive(Debug, Clone)]
pub struct Contract {
    /// Preconditions (must be true before function)
    pub requires: Vec<Condition>,
    /// Postconditions (must be true after function)
    pub ensures: Vec<Condition>,
    /// Loop invariants
    pub invariants: Vec<Invariant>,
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}

impl Contract {
    pub fn new() -> Self {
        Self {
            requires: Vec::new(),
            ensures: Vec::new(),
            invariants: Vec::new(),
        }
    }
    
    pub fn requires(mut self, condition: &str) -> Self {
        self.requires.push(Condition::parse(condition));
        self
    }
    
    pub fn ensures(mut self, condition: &str) -> Self {
        self.ensures.push(Condition::parse(condition));
        self
    }
}

/// A verification condition
#[derive(Debug, Clone)]
pub struct Condition {
    /// Expression as string
    pub expr: String,
    /// Parsed AST (simplified)
    pub ast: ConditionExpr,
}

impl Condition {
    pub fn parse(s: &str) -> Self {
        Self {
            expr: s.to_string(),
            ast: ConditionExpr::parse(s),
        }
    }
}

/// Simplified condition expression
#[derive(Debug, Clone)]
pub enum ConditionExpr {
    True,
    False,
    Var(String),
    Comparison(Box<ConditionExpr>, CompOp, Box<ConditionExpr>),
    Binary(Box<ConditionExpr>, BinOp, Box<ConditionExpr>),
    Not(Box<ConditionExpr>),
    Literal(i64),
    Result, // Special: refers to function result
}

#[derive(Debug, Clone, Copy)]
pub enum CompOp { Eq, Ne, Lt, Le, Gt, Ge }

#[derive(Debug, Clone, Copy)]
pub enum BinOp { And, Or, Add, Sub, Mul, Div }

impl ConditionExpr {
    pub fn parse(s: &str) -> Self {
        let s = s.trim();
        
        if s == "true" { return Self::True; }
        if s == "false" { return Self::False; }
        if s == "result" { return Self::Result; }
        
        if let Ok(n) = s.parse::<i64>() {
            return Self::Literal(n);
        }
        
        // Simple comparison parsing
        for (op_str, op) in [(">=", CompOp::Ge), ("<=", CompOp::Le), 
                            (">", CompOp::Gt), ("<", CompOp::Lt),
                            ("==", CompOp::Eq), ("!=", CompOp::Ne)] {
            if let Some(pos) = s.find(op_str) {
                let left = Self::parse(&s[..pos]);
                let right = Self::parse(&s[pos + op_str.len()..]);
                return Self::Comparison(Box::new(left), op, Box::new(right));
            }
        }
        
        // Default to variable
        Self::Var(s.to_string())
    }
    
    /// Evaluate with a context
    pub fn evaluate(&self, ctx: &HashMap<String, i64>) -> Option<bool> {
        match self {
            Self::True => Some(true),
            Self::False => Some(false),
            Self::Literal(_) => None,
            Self::Var(name) => ctx.get(name).map(|&v| v != 0),
            Self::Result => ctx.get("result").map(|&v| v != 0),
            Self::Comparison(l, op, r) => {
                let lv = self.eval_int(l, ctx)?;
                let rv = self.eval_int(r, ctx)?;
                Some(match op {
                    CompOp::Eq => lv == rv,
                    CompOp::Ne => lv != rv,
                    CompOp::Lt => lv < rv,
                    CompOp::Le => lv <= rv,
                    CompOp::Gt => lv > rv,
                    CompOp::Ge => lv >= rv,
                })
            }
            Self::Binary(l, op, r) => {
                match op {
                    BinOp::And => Some(l.evaluate(ctx)? && r.evaluate(ctx)?),
                    BinOp::Or => Some(l.evaluate(ctx)? || r.evaluate(ctx)?),
                    _ => None,
                }
            }
            Self::Not(e) => e.evaluate(ctx).map(|v| !v),
        }
    }
    
    fn eval_int(&self, expr: &ConditionExpr, ctx: &HashMap<String, i64>) -> Option<i64> {
        match expr {
            ConditionExpr::Literal(n) => Some(*n),
            ConditionExpr::Var(name) => ctx.get(name).copied(),
            ConditionExpr::Result => ctx.get("result").copied(),
            _ => None,
        }
    }
}

/// Loop invariant
#[derive(Debug, Clone)]
pub struct Invariant {
    pub condition: Condition,
    pub loop_id: String,
}

/// Verification result
#[derive(Debug, Clone)]
pub enum VerifyResult {
    Verified,
    Failed { condition: String, counterexample: Option<HashMap<String, i64>> },
    Unknown { reason: String },
}

/// Runtime contract checker
pub struct ContractChecker {
    contracts: HashMap<String, Contract>,
}

impl Default for ContractChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractChecker {
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
        }
    }
    
    pub fn add_contract(&mut self, function: &str, contract: Contract) {
        self.contracts.insert(function.to_string(), contract);
    }
    
    pub fn check_preconditions(&self, function: &str, args: &HashMap<String, i64>) -> VerifyResult {
        if let Some(contract) = self.contracts.get(function) {
            for req in &contract.requires {
                match req.ast.evaluate(args) {
                    Some(true) => continue,
                    Some(false) => {
                        return VerifyResult::Failed {
                            condition: req.expr.clone(),
                            counterexample: Some(args.clone()),
                        };
                    }
                    None => {
                        return VerifyResult::Unknown {
                            reason: format!("Cannot evaluate: {}", req.expr),
                        };
                    }
                }
            }
        }
        VerifyResult::Verified
    }
    
    pub fn check_postconditions(&self, function: &str, args: &HashMap<String, i64>, result: i64) -> VerifyResult {
        if let Some(contract) = self.contracts.get(function) {
            let mut ctx = args.clone();
            ctx.insert("result".to_string(), result);
            
            for ens in &contract.ensures {
                match ens.ast.evaluate(&ctx) {
                    Some(true) => continue,
                    Some(false) => {
                        return VerifyResult::Failed {
                            condition: ens.expr.clone(),
                            counterexample: Some(ctx),
                        };
                    }
                    None => {
                        return VerifyResult::Unknown {
                            reason: format!("Cannot evaluate: {}", ens.expr),
                        };
                    }
                }
            }
        }
        VerifyResult::Verified
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_condition_parse() {
        let cond = Condition::parse("x >= 0");
        let mut ctx = HashMap::new();
        ctx.insert("x".to_string(), 5);
        
        assert_eq!(cond.ast.evaluate(&ctx), Some(true));
        
        ctx.insert("x".to_string(), -1);
        assert_eq!(cond.ast.evaluate(&ctx), Some(false));
    }
    
    #[test]
    fn test_contract_checker() {
        let mut checker = ContractChecker::new();
        
        let contract = Contract::new()
            .requires("n >= 0")
            .ensures("result >= 0");
        
        checker.add_contract("sqrt", contract);
        
        let mut args = HashMap::new();
        args.insert("n".to_string(), 16);
        
        assert!(matches!(checker.check_preconditions("sqrt", &args), VerifyResult::Verified));
        assert!(matches!(checker.check_postconditions("sqrt", &args, 4), VerifyResult::Verified));
    }
}
