import { motion } from "framer-motion";
import { fadeInUp } from "../../../toolkit";

interface LoreCardProps {
  name: string;
  subtitle: string;
  description: string;
  color: string;
  stats: string;
  image: string;
  wave: string;
}

export function LoreCard({
  name,
  subtitle,
  description,
  color,
  stats,
  image,
  wave,
}: LoreCardProps) {
  return (
    <motion.div
      variants={fadeInUp}
      className="group relative overflow-hidden rounded-lg border border-fog/20 bg-stone-wall/80"
    >
      {/* Портрет врага */}
      <div className="relative h-56 overflow-hidden">
        <img
          src={image}
          alt={name}
          className="h-full w-full object-cover object-top transition-transform duration-500 group-hover:scale-105"
          loading="lazy"
        />
        {/* Градиент к телу карточки */}
        <div className="absolute inset-0 bg-gradient-to-t from-stone-wall via-transparent to-transparent" />
        {/* Цветная полоска с свечением */}
        <div
          className="absolute top-0 right-0 left-0 h-1"
          style={{ backgroundColor: color, boxShadow: `0 0 20px ${color}` }}
        />
        {/* Бейдж волны */}
        <span
          className="absolute top-3 right-3 rounded-full px-2 py-0.5 text-xs font-medium backdrop-blur-sm"
          style={{
            backgroundColor: `${color}33`,
            color,
            border: `1px solid ${color}55`,
          }}
        >
          {wave}
        </span>
      </div>

      {/* Текст */}
      <div className="p-6 pt-2">
        <p
          className="text-xs font-medium tracking-widest uppercase"
          style={{ color }}
        >
          {subtitle}
        </p>
        <h3 className="mt-1 font-display text-2xl font-bold text-gray-100">
          {name}
        </h3>
        <p className="mt-3 text-sm leading-relaxed text-fog">{description}</p>
        <p className="mt-4 font-mono text-xs text-fog/60">{stats}</p>
      </div>
    </motion.div>
  );
}
