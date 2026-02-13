import { motion } from "framer-motion";
import { Section, SectionTitle } from "../../shared/components";
import { staggerContainer } from "../../toolkit";
import { SECTION_IDS } from "../../shared/constants";
import { FeatureCard } from "./parts/FeatureCard";
import { features } from "./parts/featureData";

export function Features() {
  return (
    <Section id={SECTION_IDS.features} className="bg-stone-floor/30">
      <SectionTitle subtitle="Что делает ЧЕРТОГОН особенным">
        Рунная Мощь
      </SectionTitle>

      <motion.div
        className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3"
        variants={staggerContainer}
        initial="hidden"
        whileInView="visible"
        viewport={{ once: true, amount: 0.2 }}
      >
        {features.map((f) => (
          <FeatureCard key={f.title} {...f} />
        ))}
      </motion.div>
    </Section>
  );
}
