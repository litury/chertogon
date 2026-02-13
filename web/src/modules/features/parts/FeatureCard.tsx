import { motion } from "framer-motion";
import { fadeInUp } from "../../../toolkit";
import type { Feature } from "./featureData";

export function FeatureCard({ icon, title, description }: Feature) {
  return (
    <motion.div
      variants={fadeInUp}
      className="group rounded-lg border border-fog/20 bg-stone-floor/50 p-8 backdrop-blur-sm transition-all hover:border-gold/40 hover:bg-stone-floor/80"
    >
      <img
        src={icon}
        alt=""
        className="mb-4 h-12 w-12 rounded opacity-80 transition-opacity group-hover:opacity-100"
        loading="lazy"
      />
      <h3 className="font-display text-xl font-bold text-gold-bright">
        {title}
      </h3>
      <p className="mt-3 text-sm leading-relaxed text-fog">
        {description}
      </p>
    </motion.div>
  );
}
