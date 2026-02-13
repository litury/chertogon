import { motion } from "framer-motion";

/** Анимированный фон с рунами и туманом */
export function HeroBackground() {
  return (
    <div className="absolute inset-0 overflow-hidden">
      {/* Градиентная основа */}
      <div className="absolute inset-0 bg-gradient-to-b from-stone-wall via-stone-floor to-stone-wall" />

      {/* Анимированные руны (декоративные круги) */}
      {[...Array(6)].map((_, i) => (
        <motion.div
          key={i}
          className="absolute rounded-full border border-rune-blue/20"
          style={{
            width: 200 + i * 120,
            height: 200 + i * 120,
            left: "50%",
            top: "50%",
            x: "-50%",
            y: "-50%",
          }}
          animate={{ rotate: i % 2 === 0 ? 360 : -360, opacity: [0.1, 0.3, 0.1] }}
          transition={{ duration: 20 + i * 5, repeat: Infinity, ease: "linear" }}
        />
      ))}

      {/* Частицы-искры */}
      {[...Array(20)].map((_, i) => (
        <motion.div
          key={`spark-${i}`}
          className="absolute h-1 w-1 rounded-full bg-rune-blue/60"
          style={{
            left: `${10 + Math.random() * 80}%`,
            top: `${10 + Math.random() * 80}%`,
          }}
          animate={{
            y: [0, -30, 0],
            opacity: [0, 0.8, 0],
          }}
          transition={{
            duration: 3 + Math.random() * 4,
            repeat: Infinity,
            delay: Math.random() * 5,
          }}
        />
      ))}

      {/* Виньетка */}
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_center,transparent_30%,var(--color-stone-wall)_100%)]" />
    </div>
  );
}
