import { useRef, useEffect, useState, useCallback } from "react";
import { motion, AnimatePresence } from "framer-motion";
import type { HeroPhase } from "./useHeroGame";

interface Props {
  phase: HeroPhase;
  isMobile: boolean;
  onGameLoaded: () => void;
}

const GAME_SRC = "/game/index.html";

/** Бокс с игрой: спиннер при загрузке, iframe когда готово */
export function HeroGameBox({ phase, isMobile, onGameLoaded }: Props) {
  const iframeRef = useRef<HTMLIFrameElement>(null);
  const [isFullscreen, setIsFullscreen] = useState(false);

  const toggleFullscreen = useCallback(() => {
    const container = document.getElementById("game-box");
    if (!container) return;

    if (!document.fullscreenElement) {
      container.requestFullscreen();
      setIsFullscreen(true);
    } else {
      document.exitFullscreen();
      setIsFullscreen(false);
    }
  }, []);

  // Фокус на iframe когда игра готова
  useEffect(() => {
    if (phase === "ready") {
      const timer = setTimeout(() => iframeRef.current?.focus(), 500);
      return () => clearTimeout(timer);
    }
  }, [phase]);

  return (
    <div className="mt-4 w-full max-w-5xl md:mt-8">
      {/* Рамка игры */}
      <div
        id="game-box"
        className="relative h-[calc(100svh-120px)] overflow-hidden rounded-lg border-2 border-gold/40 bg-stone-wall shadow-[0_0_40px_-5px_var(--color-gold),inset_0_0_30px_-10px_var(--color-rune-blue)] md:h-auto md:aspect-video"
      >
        {/* Спиннер загрузки */}
        <AnimatePresence>
          {phase === "loading" && (
            <motion.div
              className="absolute inset-0 z-10 flex flex-col items-center justify-center bg-stone-wall"
              exit={{ opacity: 0 }}
              transition={{ duration: 0.5 }}
            >
              <div className="h-8 w-8 animate-spin rounded-full border-2 border-gold/30 border-t-gold" />
              <p className="mt-4 font-display text-sm text-gold/70">
                Загрузка мира...
              </p>
            </motion.div>
          )}
        </AnimatePresence>

        {/* iframe — создаётся сразу, но под спиннером */}
        <iframe
          ref={iframeRef}
          src={GAME_SRC}
          className="h-full w-full border-0"
          style={{ touchAction: "none" }}
          onLoad={onGameLoaded}
          allow="gamepad; fullscreen; autoplay"
          title="ЧЕРТОГОН — Игра"
        />
      </div>

      {/* Подсказки под боксом */}
      <AnimatePresence>
        {phase === "ready" && (
          <motion.div
            className="mt-4 flex items-center justify-between text-xs text-fog/50"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.5, delay: 0.3 }}
          >
            <span>
              {isMobile
                ? "Тапни и тяни — движение · Дальше = бег"
                : "WASD — движение · Shift — бег · Колесо — зум"}
            </span>
            <button
              onClick={toggleFullscreen}
              className="text-gold/60 transition-colors hover:text-gold"
            >
              {isFullscreen ? "Свернуть" : "На полный экран"}
            </button>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
