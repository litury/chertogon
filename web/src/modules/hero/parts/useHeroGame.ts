import { useState, useCallback, useEffect } from "react";

export type HeroPhase = "loading" | "ready" | "fallback";

/** Детектит мобильное устройство (тач без точного pointer) */
function detectMobile(): boolean {
  if (typeof window === "undefined") return false;
  return (
    /Android|iPhone|iPad|iPod/i.test(navigator.userAgent) ||
    (navigator.maxTouchPoints > 0 &&
      !window.matchMedia("(pointer: fine)").matches)
  );
}

export function useHeroGame() {
  // Начинаем с loading — iframe создаётся сразу (если WebGPU ОК)
  const [phase, setPhase] = useState<HeroPhase>("loading");
  const isMobile = detectMobile();

  // WebGL2 проверка при монтировании — если нет, переключаем на fallback
  useEffect(() => {
    const canvas = document.createElement("canvas");
    const gl = canvas.getContext("webgl2");
    if (!gl) {
      setPhase("fallback");
    }
  }, []);

  const handleGameLoaded = useCallback(() => {
    setPhase("ready");
  }, []);

  return {
    phase,
    isMobile,
    handleGameLoaded,
  };
}
