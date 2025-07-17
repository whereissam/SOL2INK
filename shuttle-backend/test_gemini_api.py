#!/usr/bin/env python3
"""Simple test to verify Gemini API key works correctly."""

import requests
import json

def test_gemini_api():
    """Test the Gemini API directly."""
    api_key = "AIzaSyBxCDT1vtb0h220IiO7AjobnjhdVVMRx-c"
    url = f"https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={api_key}"
    
    headers = {
        "Content-Type": "application/json",
    }
    
    data = {
        "contents": [{
            "parts": [{
                "text": "Hello, this is a test. Please respond with 'API working correctly'."
            }]
        }]
    }
    
    try:
        response = requests.post(url, headers=headers, json=data, timeout=10)
        print(f"Status Code: {response.status_code}")
        print(f"Response: {response.text}")
        
        if response.status_code == 200:
            result = response.json()
            if 'candidates' in result:
                print("✅ Gemini API is working correctly!")
                return True
        else:
            print("❌ Gemini API call failed")
            return False
            
    except requests.exceptions.RequestException as e:
        print(f"❌ Request failed: {e}")
        return False

if __name__ == "__main__":
    print("Testing Gemini API...")
    test_gemini_api()