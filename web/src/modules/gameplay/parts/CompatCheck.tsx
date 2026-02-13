import { useEffect, useState } from "react";
import { motion } from "framer-motion";

type Status = "checking" | "supported" | "unsupported";

export function CompatCheck({ children }: { children: React.ReactNode }) {
  const [status, setStatus] = useState<Status>("checking");

  useEffect(() => {
    async function check() {
      if (!navigator.gpu) {
        setStatus("unsupported");
        return;
      }
      try {
        const adapter = await navigator.gpu.requestAdapter();
        setStatus(adapter ? "supported" : "unsupported");
      } catch {
        setStatus("unsupported");
      }
    }
    check();
  }, []);

  if (status === "checking") return null;

  if (status === "unsupported") {
    return (
      <motion.div
        className="mx-auto max-w-xl rounded-lg border border-blood/40 bg-blood/10 p-8 text-center"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
      >
        <p className="font-display text-xl text-blood-bright">
          WebGPU не поддерживается
        </p>
        <p className="mt-3 text-sm text-fog">
          Для запуска игры нужен браузер с поддержкой WebGPU — Chrome 113+, Edge 113+ или последний Safari.
        </p>
      </motion.div>
    );
  }

  return <>{children}</>;
}
