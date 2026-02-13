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

  // WebGPU проверка при монтировании — если нет, переключаем на fallback
  useEffect(() => {
    if (isMobile) {
      setPhase("fallback");
      return;
    }

    async function check() {
      if (!navigator.gpu) {
        setPhase("fallback");
        return;
      }
      try {
        const adapter = await navigator.gpu.requestAdapter();
        if (!adapter) setPhase("fallback");
        // если adapter есть — остаёмся в "loading" (iframe уже создан)
      } catch {
        setPhase("fallback");
      }
    }
    check();
  }, [isMobile]);

  const handleGameLoaded = useCallback(() => {
    setPhase("ready");
  }, []);

  return {
    phase,
    isMobile,
    handleGameLoaded,
  };
}
