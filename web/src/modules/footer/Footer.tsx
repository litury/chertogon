import { motion } from "framer-motion";

export function Footer() {
  return (
    <footer className="relative border-t border-fog/10 bg-stone-wall px-6 py-16 text-center">
      {/* Каменный разделитель сверху */}
      <img
        src="/images/stone_divider.png"
        alt=""
        className="absolute top-0 left-1/2 h-4 w-full max-w-3xl -translate-x-1/2 -translate-y-1/2 object-cover opacity-40"
      />

      <motion.div
        initial={{ opacity: 0 }}
        whileInView={{ opacity: 1 }}
        viewport={{ once: true }}
        transition={{ duration: 0.8 }}
      >
        <p
          className="text-3xl tracking-wider text-gold/50"
          style={{ fontFamily: "var(--font-title)" }}
        >
          ЧЕРТОГОН
        </p>
        <p className="mt-3 text-sm text-parchment/40">
          Славянский Dark Fantasy &middot; Bevy Engine &middot; WebAssembly
        </p>
        <p className="mt-6 text-xs text-fog/30">
          &copy; {new Date().getFullYear()} Chertogon. Все права защищены.
        </p>
      </motion.div>
    </footer>
  );
}
