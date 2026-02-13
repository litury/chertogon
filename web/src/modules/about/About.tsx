import { motion } from "framer-motion";
import { Section } from "../../shared/components";
import { fadeInUp, fadeIn } from "../../toolkit";
import { SECTION_IDS } from "../../shared/constants";

export function About() {
  return (
    <Section id={SECTION_IDS.about}>
      <div className="grid items-center gap-12 md:grid-cols-2">
        {/* Богатырь */}
        <motion.div
          className="relative"
          variants={fadeIn}
          initial="hidden"
          whileInView="visible"
          viewport={{ once: true }}
        >
          <img
            src="/images/bogatyr_hero.png"
            alt="Богатырь с рунным мечом"
            className="mx-auto max-h-[500px] w-auto drop-shadow-[0_0_40px_var(--color-rune-blue)]"
          />
          {/* Свечение за героем */}
          <div className="absolute inset-0 -z-10 bg-[radial-gradient(ellipse_at_center,var(--color-rune-blue)_0%,transparent_60%)] opacity-20" />
        </motion.div>

        {/* Нарратив */}
        <motion.div
          variants={fadeInUp}
          initial="hidden"
          whileInView="visible"
          viewport={{ once: true }}
        >
          <h2 className="font-display text-3xl font-bold text-gold md:text-4xl">
            Порог Нави
          </h2>
          <div className="mt-2 h-px w-16 bg-gold/50" />
          <p className="mt-6 text-lg leading-relaxed text-parchment-light">
            Портал в пекло треснул. Тварей из Нави больше нечему сдерживать —
            упыри, лешие и волколаки хлынули в мир живых через Разломы на стенах
            древней арены.
          </p>
          <p className="mt-4 text-lg leading-relaxed text-parchment">
            Ты — последний богатырь. Твой рунный меч пульсирует голубым огнём, и
            каждый удар решает бросок виртуального d20. Критические попадания,
            промахи, спасброски от смерти — настоящая D&D драма в каждой секунде
            боя.
          </p>
          <p className="mt-4 text-lg leading-relaxed text-parchment/80">
            Собирай руны павших, выбирай апгрейды, комбинируй силы Перуна и
            Стрибога. Выживи как можно дольше.
          </p>
        </motion.div>
      </div>
    </Section>
  );
}
