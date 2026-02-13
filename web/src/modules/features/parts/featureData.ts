export interface Feature {
  icon: string;
  title: string;
  description: string;
}

export const features: Feature[] = [
  {
    icon: "/images/rune_3d.png",
    title: "Полное 3D",
    description:
      "Настоящий 3D top-down с 5-метровыми стенами, тенями и динамическим освещением. Не 2D-клон — арена оживает.",
  },
  {
    icon: "/images/rune_autoattack.png",
    title: "Автоатака",
    description:
      "Рунный меч бьёт сам — управляй движением, выбирай апгрейды. Просто WASD и стратегия прокачки.",
  },
  {
    icon: "/images/rune_dice.png",
    title: "D&D Механики",
    description:
      "Виртуальный d20 при каждой атаке. Криты, промахи, спасброски от смерти — драма каждую секунду.",
  },
  {
    icon: "/images/rune_evolution.png",
    title: "Рунные Эволюции",
    description:
      "13 апгрейдов + 4 эволюции. Комбинируй руны — Гром Перуна, Вихрь Стрибога, Тень Нави ждут.",
  },
  {
    icon: "/images/rune_waves.png",
    title: "Волны Нечисти",
    description:
      "Упыри, лешие, волколаки — каждая волна сильнее. Боссы Кощей и Баба Яга на горизонте.",
  },
  {
    icon: "/images/rune_browser.png",
    title: "Прямо в Браузере",
    description:
      "WebAssembly + WebGPU. Никаких установок — открыл и играешь. Telegram, десктоп, мобилка.",
  },
];
