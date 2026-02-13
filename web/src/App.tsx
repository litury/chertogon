import { Hero } from "./modules/hero";
import { About } from "./modules/about";
import { Features } from "./modules/features";
import { Lore } from "./modules/lore";
import { Footer } from "./modules/footer";

export default function App() {
  return (
    <main className="min-h-screen bg-stone-wall">
      <Hero />
      <About />
      <Features />
      <Lore />
      <Footer />
    </main>
  );
}
