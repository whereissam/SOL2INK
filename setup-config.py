#!/usr/bin/env python3
"""
Configuration setup script for SOL2INK project.
Allows easy configuration of ports and URLs across all components.
"""

import json
import os
from pathlib import Path

def update_config(backend_port=8000, frontend_port=5173, qdrant_port=6334):
    """Update configuration with new port settings."""
    
    config = {
        "backend": {
            "port": backend_port,
            "host": "localhost",
            "base_url": f"http://localhost:{backend_port}"
        },
        "frontend": {
            "port": frontend_port,
            "host": "localhost",
            "base_url": f"http://localhost:{frontend_port}"
        },
        "qdrant": {
            "port": qdrant_port,
            "host": "localhost",
            "base_url": f"http://localhost:{qdrant_port}"
        },
        "api": {
            "timeout": 30000,
            "max_retries": 3,
            "initial_retry_delay": 1000
        }
    }
    
    # Write main config file
    with open("config.json", "w") as f:
        json.dump(config, f, indent=2)
    
    # Update frontend .env file
    frontend_env_content = f"""# SOL2INK Frontend Configuration
VITE_API_BASE_URL=http://localhost:{backend_port}
VITE_MAX_RETRIES=3
VITE_INITIAL_RETRY_DELAY=1000
VITE_REQUEST_TIMEOUT=30000"""
    
    frontend_env_path = Path("SOL2INK-frontend/.env")
    frontend_env_path.write_text(frontend_env_content)
    
    # Update frontend .env.example
    frontend_env_example_path = Path("SOL2INK-frontend/.env.example")
    frontend_env_example_path.write_text(frontend_env_content)
    
    print(f"✅ Configuration updated successfully!")
    print(f"   Backend: http://localhost:{backend_port}")
    print(f"   Frontend: http://localhost:{frontend_port}")
    print(f"   Qdrant: http://localhost:{qdrant_port}")
    print(f"   Files updated: config.json, SOL2INK-frontend/.env, SOL2INK-frontend/.env.example")

def main():
    """Main configuration setup function."""
    print("🔧 SOL2INK Configuration Setup")
    print("=" * 40)
    
    # Get current configuration
    try:
        with open("config.json", "r") as f:
            current_config = json.load(f)
        current_backend_port = current_config["backend"]["port"]
        current_frontend_port = current_config["frontend"]["port"]
        current_qdrant_port = current_config["qdrant"]["port"]
        
        print(f"Current configuration:")
        print(f"  Backend port: {current_backend_port}")
        print(f"  Frontend port: {current_frontend_port}")
        print(f"  Qdrant port: {current_qdrant_port}")
        print()
        
    except FileNotFoundError:
        print("No existing configuration found. Using defaults.")
        current_backend_port = 8000
        current_frontend_port = 5173
        current_qdrant_port = 6334
    
    # Get user input
    try:
        backend_port = input(f"Enter backend port (current: {current_backend_port}): ").strip()
        backend_port = int(backend_port) if backend_port else current_backend_port
        
        frontend_port = input(f"Enter frontend port (current: {current_frontend_port}): ").strip()
        frontend_port = int(frontend_port) if frontend_port else current_frontend_port
        
        qdrant_port = input(f"Enter Qdrant port (current: {current_qdrant_port}): ").strip()
        qdrant_port = int(qdrant_port) if qdrant_port else current_qdrant_port
        
        print()
        update_config(backend_port, frontend_port, qdrant_port)
        
    except KeyboardInterrupt:
        print("\n❌ Setup cancelled by user.")
        return 1
    except ValueError:
        print("❌ Invalid port number. Please enter a valid integer.")
        return 1
    
    return 0

if __name__ == "__main__":
    exit(main())