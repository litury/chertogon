import { motion } from "framer-motion";
import {
  GAME_TITLE,
  GAME_TAGLINE,
  GAME_DESCRIPTION,
  SECTION_IDS,
} from "../../shared/constants";
import { useHeroGame } from "./parts/useHeroGame";
import { HeroBackground } from "./parts/HeroBackground";
import { HeroGameBox } from "./parts/HeroGameBox";

export function Hero() {
  const { phase, isMobile, handleGameLoaded } = useHeroGame();

  return (
    <section
      id={SECTION_IDS.hero}
      className="relative min-h-screen overflow-hidden"
    >
      <HeroBackground />

      <div className="relative z-10 flex min-h-screen flex-col items-center justify-center px-6 py-16">
        {/* Подзаголовок */}
        <motion.p
          className="mb-4 text-sm font-medium tracking-[0.3em] text-parchment/70 uppercase"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.2 }}
        >
          Славянский Dark Fantasy
        </motion.p>

        {/* Заголовок — Ruslan Display */}
        <motion.h1
          className="text-6xl font-black tracking-wider text-title-red drop-shadow-[0_0_30px_var(--color-title-glow)] md:text-8xl lg:text-9xl"
          style={{ fontFamily: "var(--font-title)" }}
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ duration: 1, delay: 0.4, ease: "easeOut" }}
        >
          {GAME_TITLE}
        </motion.h1>

        {/* Золотая орнаментальная линия */}
        <motion.div
          className="mt-2 h-px w-48 bg-gradient-to-r from-transparent via-gold to-transparent md:w-64"
          initial={{ scaleX: 0 }}
          animate={{ scaleX: 1 }}
          transition={{ duration: 1.2, delay: 0.8 }}
        />

        {/* Бокс с игрой ИЛИ fallback */}
        {phase === "fallback" ? (
          <FallbackContent isMobile={isMobile} />
        ) : (
          <HeroGameBox phase={phase} onGameLoaded={handleGameLoaded} />
        )}

        {/* Scroll hint */}
        <motion.div
          className="mt-8"
          animate={{ y: [0, 10, 0] }}
          transition={{ duration: 2, repeat: Infinity }}
        >
          <svg
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            className="text-fog/50"
          >
            <path
              d="M12 5v14M5 12l7 7 7-7"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        </motion.div>
      </div>
    </section>
  );
}

/** Fallback для мобильных / без WebGPU */
function FallbackContent({ isMobile }: { isMobile: boolean }) {
  return (
    <div className="mt-8 text-center">
      <motion.p
        className="max-w-xl text-xl font-medium text-parchment md:text-2xl"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8, delay: 0.8 }}
      >
        {GAME_TAGLINE}
      </motion.p>

      <motion.p
        className="mt-4 max-w-lg text-base text-parchment-light"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 0.8, delay: 1.2 }}
      >
        {GAME_DESCRIPTION}
      </motion.p>

      <motion.div
        className="mt-10"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.6, delay: 1.5 }}
      >
        {isMobile ? (
          <div className="max-w-md text-center">
            <p className="font-display text-lg text-torch">
              Играй на десктопе
            </p>
            <p className="mt-2 text-sm text-fog">
              Игра использует WASD + мышь. Открой эту страницу на компьютере с
              Chrome 113+.
            </p>
          </div>
        ) : (
          <div className="max-w-md rounded-lg border border-blood/40 bg-blood/10 p-6 text-center">
            <p className="font-display text-lg text-blood-bright">
              WebGPU не поддерживается
            </p>
            <p className="mt-2 text-sm text-fog">
              Для запуска нужен Chrome 113+, Edge 113+ или последний Safari.
            </p>
          </div>
        )}
      </motion.div>
    </div>
  );
}
