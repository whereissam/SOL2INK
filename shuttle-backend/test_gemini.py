#!/usr/bin/env python3
"""Test script to verify Gemini API works correctly with our configuration."""

import requests
import json
import time

# Test the /ask endpoint
def test_ask_endpoint():
    """Test the /ask endpoint with a simple query."""
    url = "http://127.0.0.1:8000/ask"
    
    # Test with POST request
    data = {
        "query": "How does the flipper contract work?"
    }
    
    try:
        response = requests.post(url, json=data, timeout=10)
        print(f"Status Code: {response.status_code}")
        print(f"Response: {response.text}")
        
        if response.status_code == 200:
            result = response.json()
            print(f"Success: {result.get('success')}")
            print(f"Data: {result.get('data', 'No data')}")
            print(f"Error: {result.get('error', 'No error')}")
        else:
            print("Request failed")
            
    except requests.exceptions.RequestException as e:
        print(f"Request failed: {e}")

if __name__ == "__main__":
    print("Testing /ask endpoint with Gemini API...")
    test_ask_endpoint()