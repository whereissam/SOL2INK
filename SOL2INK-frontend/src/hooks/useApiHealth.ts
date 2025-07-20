import { useState, useEffect } from 'react';
import { getApiUrl, API_CONFIG } from '../config/api';

export const useApiHealth = () => {
  const [isHealthy, setIsHealthy] = useState<boolean | null>(null);
  const [isChecking, setIsChecking] = useState(true);

  useEffect(() => {
    const checkHealth = async () => {
      try {
        const response = await fetch(getApiUrl(API_CONFIG.ENDPOINTS.HEALTH), {
          method: 'GET',
          headers: {
            'Content-Type': 'application/json',
          },
        });
        
        setIsHealthy(response.ok);
      } catch (error) {
        console.warn('API health check failed:', error);
        setIsHealthy(false);
      } finally {
        setIsChecking(false);
      }
    };

    checkHealth();
  }, []);

  return { isHealthy, isChecking };
};