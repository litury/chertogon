import { motion } from "framer-motion";
import { Section, SectionTitle } from "../../shared/components";
import { staggerContainer } from "../../toolkit";
import { SECTION_IDS } from "../../shared/constants";
import { LoreCard } from "./parts/LoreCard";

const enemies = [
  {
    name: "Упырь",
    subtitle: "Базовый враг",
    description:
      "Славянский зомби — основа орды. Медленный, тупой, но берёт числом. Десятки ломятся на тебя с первой волны.",
    color: "#8B0000",
    stats: "HP 20 / SPD 3.0 / DMG 10",
    image: "/images/enemy_upyr.png",
    wave: "Волна 1+",
  },
  {
    name: "Леший",
    subtitle: "Быстрый охотник",
    description:
      "Лесной дух из скрученного дерева и мха. Вдвое быстрее упыря. Заходит с флангов, опасен в стае.",
    color: "#00FFAA",
    stats: "HP 15 / SPD 6.0 / DMG 8",
    image: "/images/enemy_leshiy.png",
    wave: "Волна 3+",
  },
  {
    name: "Волколак",
    subtitle: "Хищный зверь",
    description:
      "Огромный чёрный волк с костяными талисманами. Быстрый четвероногий хищник — рвёт на части прежде чем успеешь среагировать.",
    color: "#8080A5",
    stats: "HP 12 / SPD 5.0 / DMG 15",
    image: "/images/enemy_volkolak.png",
    wave: "Волна 2+",
  },
];

export function Lore() {
  return (
    <Section id={SECTION_IDS.lore}>
      <SectionTitle subtitle="Нечисть из славянских легенд">
        Бестиарий Нави
      </SectionTitle>

      <motion.div
        className="grid gap-8 md:grid-cols-3"
        variants={staggerContainer}
        initial="hidden"
        whileInView="visible"
        viewport={{ once: true, amount: 0.2 }}
      >
        {enemies.map((e) => (
          <LoreCard key={e.name} {...e} />
        ))}
      </motion.div>

      {/* Цитата-разделитель */}
      <motion.blockquote
        className="mt-20 border-l-2 border-torch/50 py-2 pl-6 text-center text-lg italic text-torch/80 md:text-left"
        initial={{ opacity: 0, x: -20 }}
        whileInView={{ opacity: 1, x: 0 }}
        viewport={{ once: true }}
        transition={{ duration: 0.8 }}
      >
        &laquo;Два Разлома Нави раскрылись на западной стене. Из трещин ползут
        тени, и с каждой волной портал ширится...&raquo;
      </motion.blockquote>
    </Section>
  );
}
