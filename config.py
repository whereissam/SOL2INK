#!/usr/bin/env python3
"""
Configuration loader for SOL2INK project.
Loads settings from config.json and provides centralized configuration.
"""

import json
import os
from pathlib import Path

class Config:
    """Configuration loader for SOL2INK project."""
    
    def __init__(self, config_file="config.json"):
        """Load configuration from JSON file."""
        self.config_file = config_file
        self.config = self._load_config()
    
    def _load_config(self):
        """Load configuration from file."""
        config_path = Path(__file__).parent / self.config_file
        
        if not config_path.exists():
            raise FileNotFoundError(f"Configuration file not found: {config_path}")
        
        with open(config_path, 'r') as f:
            return json.load(f)
    
    @property
    def backend_url(self):
        """Get backend base URL."""
        return self.config["backend"]["base_url"]
    
    @property
    def backend_port(self):
        """Get backend port."""
        return self.config["backend"]["port"]
    
    @property
    def frontend_url(self):
        """Get frontend base URL."""
        return self.config["frontend"]["base_url"]
    
    @property
    def frontend_port(self):
        """Get frontend port."""
        return self.config["frontend"]["port"]
    
    @property
    def qdrant_url(self):
        """Get Qdrant base URL."""
        return self.config["qdrant"]["base_url"]
    
    @property
    def qdrant_port(self):
        """Get Qdrant port."""
        return self.config["qdrant"]["port"]
    
    @property
    def api_timeout(self):
        """Get API timeout in milliseconds."""
        return self.config["api"]["timeout"]
    
    @property
    def max_retries(self):
        """Get maximum retries."""
        return self.config["api"]["max_retries"]
    
    @property
    def initial_retry_delay(self):
        """Get initial retry delay in milliseconds."""
        return self.config["api"]["initial_retry_delay"]

# Global configuration instance
config = Config()

if __name__ == "__main__":
    # Test configuration loading
    print("SOL2INK Configuration:")
    print(f"Backend URL: {config.backend_url}")
    print(f"Frontend URL: {config.frontend_url}")
    print(f"Qdrant URL: {config.qdrant_url}")
    print(f"API Timeout: {config.api_timeout}ms")
    print(f"Max Retries: {config.max_retries}")