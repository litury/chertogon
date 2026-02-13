import { motion } from "framer-motion";
import { fadeInUp } from "../../../toolkit";

interface LoreCardProps {
  name: string;
  subtitle: string;
  description: string;
  color: string;
  stats: string;
}

export function LoreCard({ name, subtitle, description, color, stats }: LoreCardProps) {
  return (
    <motion.div
      variants={fadeInUp}
      className="relative overflow-hidden rounded-lg border border-fog/20 bg-stone-wall/80 p-8"
    >
      {/* Цветная полоска сверху */}
      <div className="absolute top-0 right-0 left-0 h-1" style={{ backgroundColor: color }} />

      <p className="text-xs font-medium tracking-widest uppercase" style={{ color }}>
        {subtitle}
      </p>
      <h3 className="mt-2 font-display text-2xl font-bold text-gray-100">
        {name}
      </h3>
      <p className="mt-3 text-sm leading-relaxed text-fog">
        {description}
      </p>
      <p className="mt-4 text-xs text-fog/60 font-mono">
        {stats}
      </p>
    </motion.div>
  );
}
