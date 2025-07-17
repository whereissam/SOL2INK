use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SolidityFunction {
    pub name: String,
    pub parameters: Vec<SolidityParameter>,
    pub return_type: Option<String>,
    pub visibility: String,
    pub mutability: Option<String>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SolidityParameter {
    pub name: String,
    pub type_name: String,
    pub is_indexed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SolidityStateVariable {
    pub name: String,
    pub type_name: String,
    pub visibility: String,
    pub is_mapping: bool,
    pub key_type: Option<String>,
    pub value_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SolidityEvent {
    pub name: String,
    pub parameters: Vec<SolidityParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SolidityContract {
    pub name: String,
    pub functions: Vec<SolidityFunction>,
    pub state_variables: Vec<SolidityStateVariable>,
    pub events: Vec<SolidityEvent>,
    pub custom_errors: Vec<String>,
}

pub struct SolidityParser;

impl SolidityParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_contract(&self, content: &str) -> Result<SolidityContract, String> {
        // Parse contract name
        let contract_name = self.parse_contract_name(content)?;
        
        // Parse state variables
        let state_variables = self.parse_state_variables(content)?;
        
        // Parse functions
        let functions = self.parse_functions(content)?;
        
        // Parse events
        let events = self.parse_events(content)?;
        
        // Parse custom errors
        let custom_errors = self.parse_custom_errors(content)?;
        
        Ok(SolidityContract {
            name: contract_name,
            functions,
            state_variables,
            events,
            custom_errors,
        })
    }
    
    fn parse_contract_name(&self, content: &str) -> Result<String, String> {
        let contract_re = Regex::new(r"contract\s+(\w+)").map_err(|e| format!("Regex error: {}", e))?;
        if let Some(captures) = contract_re.captures(content) {
            Ok(captures.get(1).unwrap().as_str().to_string())
        } else {
            Err("No contract name found".to_string())
        }
    }
    
    fn parse_state_variables(&self, content: &str) -> Result<Vec<SolidityStateVariable>, String> {
        let mut variables = Vec::new();
        
        // Parse regular state variables
        let var_re = Regex::new(r"(\w+)\s+(public|private|internal)\s+(\w+);").map_err(|e| format!("Regex error: {}", e))?;
        for captures in var_re.captures_iter(content) {
            let type_name = captures.get(1).unwrap().as_str();
            let visibility = captures.get(2).unwrap().as_str();
            let name = captures.get(3).unwrap().as_str();
            
            variables.push(SolidityStateVariable {
                name: name.to_string(),
                type_name: type_name.to_string(),
                visibility: visibility.to_string(),
                is_mapping: false,
                key_type: None,
                value_type: None,
            });
        }
        
        // Parse mappings
        let mapping_re = Regex::new(r"mapping\((\w+)\s*=>\s*(\w+)\)\s+(public|private|internal)\s+(\w+);").map_err(|e| format!("Regex error: {}", e))?;
        for captures in mapping_re.captures_iter(content) {
            let key_type = captures.get(1).unwrap().as_str();
            let value_type = captures.get(2).unwrap().as_str();
            let visibility = captures.get(3).unwrap().as_str();
            let name = captures.get(4).unwrap().as_str();
            
            variables.push(SolidityStateVariable {
                name: name.to_string(),
                type_name: format!("mapping({} => {})", key_type, value_type),
                visibility: visibility.to_string(),
                is_mapping: true,
                key_type: Some(key_type.to_string()),
                value_type: Some(value_type.to_string()),
            });
        }
        
        // Parse nested mappings
        let nested_mapping_re = Regex::new(r"mapping\((\w+)\s*=>\s*mapping\((\w+)\s*=>\s*(\w+)\)\)\s+(public|private|internal)\s+(\w+);").map_err(|e| format!("Regex error: {}", e))?;
        for captures in nested_mapping_re.captures_iter(content) {
            let key_type = captures.get(1).unwrap().as_str();
            let inner_key_type = captures.get(2).unwrap().as_str();
            let value_type = captures.get(3).unwrap().as_str();
            let visibility = captures.get(4).unwrap().as_str();
            let name = captures.get(5).unwrap().as_str();
            
            variables.push(SolidityStateVariable {
                name: name.to_string(),
                type_name: format!("mapping({} => mapping({} => {}))", key_type, inner_key_type, value_type),
                visibility: visibility.to_string(),
                is_mapping: true,
                key_type: Some(key_type.to_string()),
                value_type: Some(format!("mapping({} => {})", inner_key_type, value_type)),
            });
        }
        
        Ok(variables)
    }
    
    fn parse_functions(&self, content: &str) -> Result<Vec<SolidityFunction>, String> {
        let mut functions = Vec::new();
        
        // Parse constructor - handle multiline with dot-all modifier
        let constructor_re = Regex::new(r"(?s)constructor\s*\((.*?)\)\s*\{(.*?)\}").map_err(|e| format!("Regex error: {}", e))?;
        if let Some(captures) = constructor_re.captures(content) {
            let params_str = captures.get(1).unwrap().as_str();
            let body = captures.get(2).unwrap().as_str();
            
            let parameters = self.parse_parameters(params_str)?;
            
            functions.push(SolidityFunction {
                name: "constructor".to_string(),
                parameters,
                return_type: None,
                visibility: "public".to_string(),
                mutability: None,
                body: body.to_string(),
            });
        }
        
        // Parse regular functions - handle multiline with dot-all modifier
        let function_re = Regex::new(r"(?s)function\s+(\w+)\s*\((.*?)\)\s+(public|private|internal|external)(?:\s+(view|pure|payable))?\s*(?:returns\s*\(([^)]*)\))?\s*\{(.*?)\}").map_err(|e| format!("Regex error: {}", e))?;
        for captures in function_re.captures_iter(content) {
            let name = captures.get(1).unwrap().as_str();
            let params_str = captures.get(2).unwrap().as_str();
            let visibility = captures.get(3).unwrap().as_str();
            let mutability = captures.get(4).map(|m| m.as_str().to_string());
            let return_type = captures.get(5).map(|r| {
                // Extract just the type part from "type name" format
                let return_str = r.as_str().trim();
                if let Some(space_pos) = return_str.find(' ') {
                    return_str[..space_pos].to_string()
                } else {
                    return_str.to_string()
                }
            });
            let body = captures.get(6).unwrap().as_str();
            
            let parameters = self.parse_parameters(params_str)?;
            
            functions.push(SolidityFunction {
                name: name.to_string(),
                parameters,
                return_type,
                visibility: visibility.to_string(),
                mutability,
                body: body.to_string(),
            });
        }
        
        Ok(functions)
    }
    
    fn parse_parameters(&self, params_str: &str) -> Result<Vec<SolidityParameter>, String> {
        let mut parameters = Vec::new();
        
        if params_str.trim().is_empty() {
            return Ok(parameters);
        }
        
        // Split by comma and parse each parameter
        for param in params_str.split(',') {
            let param = param.trim();
            if param.is_empty() {
                continue;
            }
            
            // Parse parameter format: "type name" or "type memory name" etc.
            let parts: Vec<&str> = param.split_whitespace().collect();
            if parts.len() >= 2 {
                let type_name = if parts.len() > 2 {
                    // Handle "string memory _name" format
                    parts[0].to_string()
                } else {
                    parts[0].to_string()
                };
                let name = parts.last().unwrap().to_string();
                
                parameters.push(SolidityParameter {
                    name,
                    type_name,
                    is_indexed: false,
                });
            }
        }
        
        Ok(parameters)
    }
    
    fn parse_events(&self, content: &str) -> Result<Vec<SolidityEvent>, String> {
        let mut events = Vec::new();
        
        let event_re = Regex::new(r"event\s+(\w+)\s*\((.*?)\);").map_err(|e| format!("Regex error: {}", e))?;
        for captures in event_re.captures_iter(content) {
            let name = captures.get(1).unwrap().as_str();
            let params_str = captures.get(2).unwrap().as_str();
            
            let parameters = self.parse_event_parameters(params_str)?;
            
            events.push(SolidityEvent {
                name: name.to_string(),
                parameters,
            });
        }
        
        Ok(events)
    }
    
    fn parse_event_parameters(&self, params_str: &str) -> Result<Vec<SolidityParameter>, String> {
        let mut parameters = Vec::new();
        
        if params_str.trim().is_empty() {
            return Ok(parameters);
        }
        
        // Split by comma and parse each parameter
        for param in params_str.split(',') {
            let param = param.trim();
            if param.is_empty() {
                continue;
            }
            
            // Check if indexed
            let is_indexed = param.contains("indexed");
            
            // Parse parameter format: "type indexed name" or "type name"
            let parts: Vec<&str> = param.split_whitespace().collect();
            if parts.len() >= 2 {
                let type_name = parts[0].to_string();
                let name = parts.last().unwrap().to_string();
                
                parameters.push(SolidityParameter {
                    name,
                    type_name,
                    is_indexed,
                });
            }
        }
        
        Ok(parameters)
    }
    
    fn parse_custom_errors(&self, content: &str) -> Result<Vec<String>, String> {
        let mut errors = Vec::new();
        
        let error_re = Regex::new(r"error\s+(\w+)\s*\(.*?\);").map_err(|e| format!("Regex error: {}", e))?;
        for captures in error_re.captures_iter(content) {
            let name = captures.get(1).unwrap().as_str();
            errors.push(name.to_string());
        }
        
        Ok(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_solidity_erc20_contract_and_extract_functions() {
        let solidity_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

contract SimpleERC20 {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;

    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    error InsufficientBalance(address account, uint256 requested, uint256 available);
    error InsufficientAllowance(address spender, uint256 requested, uint256 available);

    constructor(
        string memory _name,
        string memory _symbol,
        uint8 _decimals,
        uint256 _totalSupply
    ) {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;
        totalSupply = _totalSupply;
        balanceOf[msg.sender] = _totalSupply;
        emit Transfer(address(0), msg.sender, _totalSupply);
    }

    function transfer(address to, uint256 value) public returns (bool success) {
        if (balanceOf[msg.sender] < value) {
            revert InsufficientBalance(msg.sender, value, balanceOf[msg.sender]);
        }
        
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        emit Transfer(msg.sender, to, value);
        return true;
    }

    function approve(address spender, uint256 value) public returns (bool success) {
        allowance[msg.sender][spender] = value;
        emit Approval(msg.sender, spender, value);
        return true;
    }
}
"#;

        let parser = SolidityParser::new();
        let result = parser.parse_contract(solidity_code);

        assert!(result.is_ok());
        let contract = result.unwrap();
        
        // Verify contract name
        assert_eq!(contract.name, "SimpleERC20");
        
        // Verify state variables
        assert_eq!(contract.state_variables.len(), 6);
        
        // Check for specific state variables
        let name_var = contract.state_variables.iter()
            .find(|v| v.name == "name")
            .expect("name variable should exist");
        assert_eq!(name_var.type_name, "string");
        assert_eq!(name_var.visibility, "public");
        
        let balance_mapping = contract.state_variables.iter()
            .find(|v| v.name == "balanceOf")
            .expect("balanceOf mapping should exist");
        assert!(balance_mapping.is_mapping);
        assert_eq!(balance_mapping.key_type, Some("address".to_string()));
        assert_eq!(balance_mapping.value_type, Some("uint256".to_string()));
        
        // Verify functions
        assert_eq!(contract.functions.len(), 3); // constructor + transfer + approve
        
        // Check transfer function
        let transfer_fn = contract.functions.iter()
            .find(|f| f.name == "transfer")
            .expect("transfer function should exist");
        assert_eq!(transfer_fn.parameters.len(), 2);
        assert_eq!(transfer_fn.parameters[0].name, "to");
        assert_eq!(transfer_fn.parameters[0].type_name, "address");
        assert_eq!(transfer_fn.parameters[1].name, "value");
        assert_eq!(transfer_fn.parameters[1].type_name, "uint256");
        assert_eq!(transfer_fn.return_type, Some("bool".to_string()));
        assert_eq!(transfer_fn.visibility, "public");
        
        // Verify events
        assert_eq!(contract.events.len(), 2);
        
        let transfer_event = contract.events.iter()
            .find(|e| e.name == "Transfer")
            .expect("Transfer event should exist");
        assert_eq!(transfer_event.parameters.len(), 3);
        assert!(transfer_event.parameters[0].is_indexed); // from
        assert!(transfer_event.parameters[1].is_indexed); // to
        assert!(!transfer_event.parameters[2].is_indexed); // value
        
        // Verify custom errors
        assert_eq!(contract.custom_errors.len(), 2);
        assert!(contract.custom_errors.contains(&"InsufficientBalance".to_string()));
        assert!(contract.custom_errors.contains(&"InsufficientAllowance".to_string()));
    }
}