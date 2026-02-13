import { motion } from "framer-motion";
import { fadeInUp } from "../../../toolkit";
import type { Feature } from "./featureData";

export function FeatureCard({ icon, title, description }: Feature) {
  return (
    <motion.div
      variants={fadeInUp}
      className="group rounded-lg border border-fog/20 bg-stone-floor/50 p-8 backdrop-blur-sm transition-all hover:border-rune-blue/40 hover:bg-stone-floor/80"
    >
      <span
        className="mb-4 block text-4xl"
        dangerouslySetInnerHTML={{ __html: icon }}
      />
      <h3 className="font-display text-xl font-bold text-rune-glow">
        {title}
      </h3>
      <p className="mt-3 text-sm leading-relaxed text-fog">
        {description}
      </p>
    </motion.div>
  );
}
