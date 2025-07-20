import { useEffect, useState } from 'react';

export const useMobileOptimizations = () => {
  const [isMobile, setIsMobile] = useState(false);
  const [viewportHeight, setViewportHeight] = useState(window.innerHeight);
  const [isKeyboardOpen, setIsKeyboardOpen] = useState(false);

  useEffect(() => {
    const checkMobile = () => {
      const mobile = window.innerWidth < 640 || 'ontouchstart' in window;
      setIsMobile(mobile);
    };

    const handleResize = () => {
      const newHeight = window.innerHeight;
      const heightDiff = viewportHeight - newHeight;
      
      // Detect virtual keyboard on mobile
      if (isMobile && heightDiff > 150) {
        setIsKeyboardOpen(true);
      } else {
        setIsKeyboardOpen(false);
      }
      
      setViewportHeight(newHeight);
    };

    checkMobile();
    window.addEventListener('resize', handleResize);
    window.addEventListener('orientationchange', checkMobile);

    return () => {
      window.removeEventListener('resize', handleResize);
      window.removeEventListener('orientationchange', checkMobile);
    };
  }, [viewportHeight, isMobile]);

  // Prevent scrolling on iOS when chat is open
  const preventBackgroundScroll = (prevent: boolean) => {
    if (isMobile) {
      document.body.style.overflow = prevent ? 'hidden' : '';
      document.body.style.position = prevent ? 'fixed' : '';
      document.body.style.width = prevent ? '100%' : '';
    }
  };

  return {
    isMobile,
    viewportHeight,
    isKeyboardOpen,
    preventBackgroundScroll,
  };
};