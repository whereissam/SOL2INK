#!/usr/bin/env python3
"""
Script to fix missing 'object' fields in ApiResponse instances
"""

import re

def fix_api_responses(file_path):
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Pattern to match ApiResponse instances without object field
    pattern = r'ApiResponse\s*\{\s*success:\s*(true|false),\s*data:\s*([^,]+),\s*error:\s*([^}]+)\s*\}'
    
    def replacement(match):
        success = match.group(1)
        data = match.group(2)
        error = match.group(3)
        
        object_name = "response"
        if "true" in success:
            if "count" in data.lower():
                object_name = "count"
            elif "strategies" in data.lower():
                object_name = "strategies"
            elif "statistics" in data.lower():
                object_name = "statistics"
        else:
            object_name = "error"
        
        return f'ApiResponse {{\n                object: "{object_name}".to_string(),\n                success: {success},\n                data: {data},\n                error: {error}\n            }}'
    
    fixed_content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
    
    with open(file_path, 'w') as f:
        f.write(fixed_content)
    
    print(f"Fixed API responses in {file_path}")

if __name__ == "__main__":
    fix_api_responses("src/main.rs")