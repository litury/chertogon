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

      <div className="relative z-10 flex min-h-screen flex-col items-center justify-start px-6 py-4 pt-[max(1rem,env(safe-area-inset-top))] md:justify-center md:py-16">
        {/* Подзаголовок */}
        <motion.p
          className="mb-1 text-sm font-medium tracking-[0.3em] text-parchment/70 uppercase md:mb-4"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8, delay: 0.2 }}
        >
          Славянский Dark Fantasy
        </motion.p>

        {/* Заголовок — кованый огненный эффект */}
        <motion.div
          className="forged-title text-4xl font-black tracking-wider md:text-8xl lg:text-9xl"
          style={{ fontFamily: "var(--font-title)" }}
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ duration: 1, delay: 0.4, ease: "easeOut" }}
        >
          {/* 3D глубина — тень позади */}
          <span className="forged-title-depth" aria-hidden="true">
            {GAME_TITLE}
          </span>
          {/* Огненное свечение — пульсирует */}
          <span className="forged-title-glow" aria-hidden="true">
            {GAME_TITLE}
          </span>
          {/* Основной текст — металлический градиент */}
          <h1 className="forged-title-text">{GAME_TITLE}</h1>
        </motion.div>

        {/* Золотая орнаментальная линия */}
        <motion.div
          className="mt-2 h-px w-48 bg-gradient-to-r from-transparent via-gold to-transparent md:w-64"
          initial={{ scaleX: 0 }}
          animate={{ scaleX: 1 }}
          transition={{ duration: 1.2, delay: 0.8 }}
        />

        {/* Бокс с игрой ИЛИ fallback */}
        {phase === "fallback" ? (
          <FallbackContent />
        ) : (
          <HeroGameBox phase={phase} isMobile={isMobile} onGameLoaded={handleGameLoaded} />
        )}

        {/* Scroll hint */}
        <motion.div
          className="mt-8 hidden md:block"
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

/** Fallback — браузер без WebGL2 */
function FallbackContent() {
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
        <div className="max-w-md rounded-lg border border-blood/40 bg-blood/10 p-6 text-center">
          <p className="font-display text-lg text-blood-bright">
            WebGL2 не поддерживается
          </p>
          <p className="mt-2 text-sm text-fog">
            Обновите браузер до последней версии.
          </p>
        </div>
      </motion.div>
    </div>
  );
}
