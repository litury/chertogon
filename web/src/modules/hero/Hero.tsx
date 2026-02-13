import { SECTION_IDS } from "../../shared/constants";
import { HeroBackground } from "./parts/HeroBackground";
import { HeroTitle } from "./parts/HeroTitle";

export function Hero() {
  return (
    <section id={SECTION_IDS.hero} className="relative min-h-screen overflow-hidden">
      <HeroBackground />
      <HeroTitle />
    </section>
  );
}
