import { Hero } from "./modules/hero";
import { Features } from "./modules/features";
import { Lore } from "./modules/lore";
import { Gameplay } from "./modules/gameplay";
import { Footer } from "./modules/footer";

export default function App() {
  return (
    <main className="min-h-screen bg-stone-wall">
      <Hero />
      <Features />
      <Lore />
      <Gameplay />
      <Footer />
    </main>
  );
}
