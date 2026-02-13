import { motion } from "framer-motion";
import { GAME_TITLE, GAME_TAGLINE, GAME_DESCRIPTION, SECTION_IDS } from "../../../shared/constants";

export function HeroTitle() {
  return (
    <div className="relative z-10 flex min-h-screen flex-col items-center justify-center text-center">
      {/* Подзаголовок */}
      <motion.p
        className="mb-4 text-sm font-medium tracking-[0.3em] text-rune-blue/70 uppercase"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8, delay: 0.2 }}
      >
        Славянский Dark Fantasy
      </motion.p>

      {/* Главный заголовок */}
      <motion.h1
        className="font-display text-7xl font-black tracking-wider text-rune-blue drop-shadow-[0_0_30px_var(--color-rune-blue)] md:text-9xl"
        initial={{ opacity: 0, scale: 0.8 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ duration: 1, delay: 0.4, ease: "easeOut" }}
      >
        {GAME_TITLE}
      </motion.h1>

      {/* Тэглайн */}
      <motion.p
        className="mt-6 max-w-xl text-xl font-medium text-soul-green/90 md:text-2xl"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8, delay: 0.8 }}
      >
        {GAME_TAGLINE}
      </motion.p>

      {/* Описание */}
      <motion.p
        className="mt-4 max-w-lg text-base text-fog"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 0.8, delay: 1.2 }}
      >
        {GAME_DESCRIPTION}
      </motion.p>

      {/* CTA кнопка */}
      <motion.a
        href={`#${SECTION_IDS.gameplay}`}
        className="mt-10 inline-block rounded-sm border-2 border-rune-blue bg-rune-blue/10 px-10 py-4 font-display text-lg font-bold tracking-wider text-rune-blue uppercase transition-all hover:bg-rune-blue/20 hover:shadow-[0_0_30px_var(--color-rune-blue)]"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.6, delay: 1.5 }}
        whileHover={{ scale: 1.05 }}
        whileTap={{ scale: 0.97 }}
      >
        Играть в браузере
      </motion.a>

      {/* Scroll hint */}
      <motion.div
        className="absolute bottom-10 left-1/2 -translate-x-1/2"
        animate={{ y: [0, 10, 0] }}
        transition={{ duration: 2, repeat: Infinity }}
      >
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" className="text-fog/50">
          <path d="M12 5v14M5 12l7 7 7-7" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
        </svg>
      </motion.div>
    </div>
  );
}
