import { useState, useCallback } from "react";
import { motion, AnimatePresence } from "framer-motion";

/** Встраивает WASM-игру через iframe с ленивой загрузкой */
export function GameCanvas() {
  const [isLoading, setIsLoading] = useState(false);
  const [isLoaded, setIsLoaded] = useState(false);
  const [isFullscreen, setIsFullscreen] = useState(false);

  const handlePlay = useCallback(() => {
    setIsLoading(true);
  }, []);

  const handleIframeLoad = useCallback(() => {
    setIsLoading(false);
    setIsLoaded(true);
  }, []);

  const toggleFullscreen = useCallback(() => {
    const container = document.getElementById("game-frame-container");
    if (!container) return;

    if (!document.fullscreenElement) {
      container.requestFullscreen();
      setIsFullscreen(true);
    } else {
      document.exitFullscreen();
      setIsFullscreen(false);
    }
  }, []);

  return (
    <div className="relative mx-auto max-w-4xl">
      {/* Рамка iframe */}
      <div
        id="game-frame-container"
        className="relative aspect-video overflow-hidden rounded-lg border-2 border-rune-blue/30 bg-stone-wall shadow-[0_0_60px_-10px_var(--color-rune-blue)]"
      >
        {/* Плейсхолдер до нажатия */}
        <AnimatePresence>
          {!isLoading && !isLoaded && (
            <motion.div
              className="absolute inset-0 flex flex-col items-center justify-center bg-gradient-to-b from-stone-floor to-stone-wall"
              exit={{ opacity: 0 }}
              transition={{ duration: 0.3 }}
            >
              <p className="mb-6 font-display text-2xl text-rune-blue/60">
                ЧЕРТОГОН
              </p>
              <button
                onClick={handlePlay}
                className="group relative rounded-sm border-2 border-rune-blue bg-rune-blue/10 px-12 py-5 font-display text-xl font-bold tracking-wider text-rune-blue uppercase transition-all hover:bg-rune-blue/25 hover:shadow-[0_0_40px_var(--color-rune-blue)]"
              >
                <span className="relative z-10">Запустить Игру</span>
              </button>
              <p className="mt-4 text-xs text-fog/50">
                Требуется WebGPU (Chrome 113+ / Edge 113+)
              </p>
            </motion.div>
          )}
        </AnimatePresence>

        {/* Loading */}
        {isLoading && !isLoaded && (
          <div className="absolute inset-0 flex flex-col items-center justify-center bg-stone-wall">
            <div className="h-8 w-8 animate-spin rounded-full border-2 border-rune-blue/30 border-t-rune-blue" />
            <p className="mt-4 font-display text-sm text-rune-blue/70">
              Загрузка WASM...
            </p>
          </div>
        )}

        {/* Iframe */}
        {(isLoading || isLoaded) && (
          <iframe
            src="/game/index.html"
            className="h-full w-full border-0"
            onLoad={handleIframeLoad}
            allow="gamepad; fullscreen; autoplay"
            title="ЧЕРТОГОН — Игра"
          />
        )}
      </div>

      {/* Контролы под iframe */}
      {isLoaded && (
        <div className="mt-4 flex items-center justify-between text-xs text-fog/50">
          <span>WASD — движение / Shift — бег / Колесо — зум</span>
          <button
            onClick={toggleFullscreen}
            className="text-rune-blue/60 transition-colors hover:text-rune-blue"
          >
            {isFullscreen ? "Выйти из полного экрана" : "На полный экран"}
          </button>
        </div>
      )}
    </div>
  );
}
