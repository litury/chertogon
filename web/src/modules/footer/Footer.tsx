import { motion } from "framer-motion";

export function Footer() {
  return (
    <footer className="border-t border-fog/10 bg-stone-wall px-6 py-16 text-center">
      <motion.div
        initial={{ opacity: 0 }}
        whileInView={{ opacity: 1 }}
        viewport={{ once: true }}
        transition={{ duration: 0.8 }}
      >
        <p className="font-display text-2xl font-bold tracking-wider text-rune-blue/50">
          ЧЕРТОГОН
        </p>
        <p className="mt-3 text-sm text-fog/40">
          Славянский Dark Fantasy &middot; Bevy Engine &middot; WebAssembly
        </p>
        <p className="mt-6 text-xs text-fog/30">
          &copy; {new Date().getFullYear()} Chertogon. Все права защищены.
        </p>
      </motion.div>
    </footer>
  );
}
