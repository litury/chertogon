import { motion } from "framer-motion";

/** Атмосферный фон с генерированным изображением арены, туманом и искрами */
export function HeroBackground() {
  return (
    <div className="absolute inset-0 overflow-hidden">
      {/* Генерированный фон арены */}
      <div
        className="absolute inset-0 bg-cover bg-center"
        style={{ backgroundImage: "url('/images/hero_bg.jpg')" }}
      />

      {/* Тёмный оверлей для читаемости текста */}
      <div className="absolute inset-0 bg-stone-wall/60" />

      {/* Анимированные слои тумана */}
      <motion.div
        className="absolute inset-0 opacity-15"
        style={{
          background:
            "linear-gradient(90deg, transparent 0%, var(--color-fog) 30%, transparent 60%)",
        }}
        animate={{ x: ["-100%", "100%"] }}
        transition={{ duration: 25, repeat: Infinity, ease: "linear" }}
      />
      <motion.div
        className="absolute inset-0 opacity-8"
        style={{
          background:
            "linear-gradient(90deg, transparent 20%, var(--color-rune-blue) 50%, transparent 80%)",
        }}
        animate={{ x: ["100%", "-100%"] }}
        transition={{ duration: 35, repeat: Infinity, ease: "linear" }}
      />

      {/* Частицы-искры с свечением */}
      {[...Array(15)].map((_, i) => (
        <motion.div
          key={`spark-${i}`}
          className="absolute h-[2px] w-[2px] rounded-full"
          style={{
            left: `${10 + Math.random() * 80}%`,
            top: `${10 + Math.random() * 80}%`,
            backgroundColor:
              i % 3 === 0
                ? "var(--color-torch)"
                : i % 3 === 1
                  ? "var(--color-gold)"
                  : "var(--color-rune-blue)",
            boxShadow: `0 0 6px 2px ${
              i % 3 === 0
                ? "var(--color-torch)"
                : i % 3 === 1
                  ? "var(--color-gold)"
                  : "var(--color-rune-blue)"
            }`,
          }}
          animate={{ y: [0, -40, 0], opacity: [0, 0.9, 0] }}
          transition={{
            duration: 3 + Math.random() * 4,
            repeat: Infinity,
            delay: Math.random() * 5,
          }}
        />
      ))}

      {/* Усиленная виньетка */}
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_center,transparent_20%,var(--color-stone-wall)_85%)]" />
    </div>
  );
}
